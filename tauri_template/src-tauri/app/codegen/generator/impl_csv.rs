use std::borrow::Cow;
use std::io::Write;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use convert_case::{Casing, Case};
use proc_macro2::{TokenStream, Span};
use quote::{quote, format_ident};
use super::type_inference::{ts_to_rust_type};
use super::load::{load_yaml};
use crate::codegen::SetupConfig;
use crate::codegen::types::{Csv2DB, Csv2DBConfig};

use std::fs::OpenOptions;

fn debug_store_info(file_path: &str, info: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "[DEBUG] {}", info)?;

    Ok(())
}

struct FieldSchema {
    pub name: syn::Ident,
    pub ty: syn::Type,
    pub type_str: String,
    pub nullable: bool,
}

pub fn make_csv_ingestion(
    setup: &SetupConfig,
) -> Result<(), Box<dyn std::error::Error>>{
    let schemas = load_all_schemas(
        &setup.input_path.csv, &setup.input_path.db_schema
    )?;
    let output_path = &format!(
        "{}/{}/{}",
        setup.output_path.target_dir, setup.output_path.prefix, setup.output_path.dto
    ) ;
    write_all_schemas(schemas, &output_path)?;
    Ok(())
}

fn write_all_schemas(
    schemas: HashMap<String, Csv2DB>, 
    output_path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let output_base_dir = Path::new(output_path);
    let output_dir = output_base_dir.join("csv");
    let mut mods = String::from("pub mod base_insert_trait;\n");
    std::fs::create_dir_all(&output_dir)?;
    write_base_insert(&output_dir)?;
    append_csv_mod(&output_base_dir)?;
    for (fname, schema) in &schemas {
        let tokens = generate_struct(schema)?;
        let output = prettyplease::unparse(
            &syn::parse_file(&tokens.to_string())?
        );
        let file_path = output_dir.join(format!("{fname}.rs"));
        let mut file = std::fs::File::create(file_path)?;
        mods = format!("{mods}pub mod {fname};\n");
        file.write(output.as_bytes())?;
    }
    let mod_file_path = output_dir.join("mod.rs");
    let mut mod_file = std::fs::File::create(mod_file_path)?;
    mod_file.write(mods.as_bytes())?;
    Ok(())
}

fn append_csv_mod (
    output_dir: &Path
)  -> Result<(), Box<dyn std::error::Error>> {
    let mod_file_path = output_dir.join("mod.rs");
    let mods = String::from("pub mod csv;\n");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(mod_file_path)?;
    writeln!(file, "{}", mods)?;
    Ok(())
}

fn write_base_insert (
    output_dir: &Path
) -> Result<(), Box<dyn std::error::Error>> {
    let output_file = output_dir.join("base_insert_trait.rs");
    let mut file = std::fs::File::create(output_file)?;
    let tokens = quote! {
        use sqlx::{Sqlite, QueryBuilder};
        pub trait BulkInsert {
            type Item;
            type Context;

            fn insert<'a>(
                qb: &mut QueryBuilder<Sqlite>,
                items: &'a [Self::Item],
                ctx: &Self::Context,
            );
        }
    };
    let output = prettyplease::unparse(
        &syn::parse_file(&tokens.to_string())?
    );
    file.write(output.as_bytes())?;
    Ok(())
}

fn load_all_schemas(
    csv_path: &str, db_path: &str
) -> Result<HashMap<String, Csv2DB>, Box<dyn std::error::Error>> {
    let mut res = HashMap::new();
    let schema_dir = Path::new(csv_path);
    let db_dir = Path::new(db_path);
    for entry in std::fs::read_dir(schema_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !(
            path
            .extension()
            .and_then(|s| s.to_str()) == Some("yaml")
        ) { continue; }
        let name = match path.file_stem() {
            Some(s) => s.to_string_lossy().to_string(),
            None => continue,
        };
        let config: Csv2DBConfig = load_yaml(path)?;
        let db_fname = Path::new(
            &config.target
        ).with_extension("yaml");
        res.insert(name, Csv2DB {
            config,
            data: load_yaml(db_dir.join(db_fname))?
        });

    }
    Ok(res)
}

fn generate_struct(
    schema: &Csv2DB 
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let struct_name_ref =  &schema.config.target.to_case(Case::Pascal);
    let csv_name = format_ident!("{}", format!("{struct_name_ref}Ingestion"));
    let column_names = format_ident!("{}", format!("{struct_name_ref}Columns"));
    let insert_query = format_ident!("{}", format!("{struct_name_ref}BulkInsert"));

    let exclude: HashSet<_> = schema.config.exclude.iter().collect();

    let filtered: Vec<_> =  schema.data.fields.iter()
    .filter(|f| !exclude.contains(&f.name))
    .map(|f| Ok(FieldSchema {
        name: format_ident!("{}", f.name),
        ty: syn::parse_str(&ts_to_rust_type(&f.ty))?,
        type_str: ts_to_rust_type(&f.ty).to_string(),
        nullable: f.nullable
    }))
    .collect::<Result<_, syn::Error>>()?; 

    let struct_fields: Vec<_> = filtered.iter()
    .map(|f| {
        let name = format_ident!("{}", f.name);
        let base_ty = f.ty.clone();
        let ty: syn::Type = if f.nullable {
            syn::parse_quote!(Option<#base_ty>)
        } else {
            base_ty
        };
        Ok(quote! {
            pub #name: #ty
        })
    }).collect::<Result<_, syn::Error>>()?;

    let bind_fields: Vec<_> = filtered.iter()
    .map(|f| {
        let name = f.name.clone();

        // let bind = check_bind_type(&f.ty);
        // if bind.is_empty() {
        if f.type_str == "u64" {
            quote! {
                .push_bind(&cols.#name.get(row).map(|x| x as i64))
            }
        } else {
            quote! {
                .push_bind(&cols.#name.get(row))
            }
        }
        // } else {
        //     quote! {
        //         .push_bind(&(cols.#name.get(row) #bind))
        //     }
        // }
    }).collect();
    
    let load_schemas_fields: Vec<_> = filtered.iter()
    .map(|f| {
        let data = rust_type_to_dtype(&f.type_str);
        let name = syn::LitStr::new(&f.name.to_string(), Span::call_site());
        // Ok(quote! {Field::new(PlSmallStr::from_str(#name), #data)})
        Ok(quote! {Field::new(#name.into(), #data)})
    }).collect::<Result<_, syn::Error>>()?;

    let optional_fields: Vec<_> = schema.config.optional_fields
    .iter().map(|f| {
        let field = syn::LitStr::new(&f, Span::call_site());
        quote! { #field }
    }).collect();
    
    let extra_identify = format_ident!("{}Ctx", struct_name_ref);
    let extra_para_type = if schema.config.add_bind_values.is_empty() {
        quote! {()}
    } else {
        quote! {#extra_identify}
    };
    let extra_para_name = if schema.config.add_bind_values.is_empty() {
        quote! {_}
    } else {
        quote! {ctx}
    };

    let csv_column: Vec<_> = filtered.iter().map(|f| {
        let name = f.name.clone();
        let col_type = rust_type_to_chunked(f.type_str.as_str()).unwrap_or("StringChunked");
        let ty: syn::Type = syn::parse_str(col_type)?;
        Ok(quote! {pub #name: &'a #ty})
    }).collect::<Result<_, syn::Error>>()?;

    let prepare_csv_fields: Vec<_> = filtered.iter().map(|f| {
        let name = f.name.clone();
        let col_type = rust_type_df_cast(f.type_str.as_str());
        let ty = format_ident!("{}", &col_type);
        let field_name = syn::LitStr::new(&f.name.to_string(), Span::call_site());
        Ok(quote! { #name: df.column(#field_name)?.#ty()? })
    }).collect::<Result<_, syn::Error>>()?;

    let extra_para_struct = if schema.config.add_bind_values.is_empty() {
        quote! {}
    } else {
        let fields: Vec<_> = schema.config.add_bind_values.iter()
        .map(|f| {
            let name: syn::Ident = syn::parse_str(&f.name)?;
            let base_ty: syn::Type = syn::parse_str(&f.ty)?;
            let ty: syn::Type = if f.nullable {
                syn::parse_quote!(Option<#base_ty>)
            } else {
                base_ty
            };

            Ok(quote! {pub #name: #ty})
        }).collect::<Result<_, syn::Error>>()?;
        quote! {
            pub struct #extra_identify {
                #(#fields),*
            }
        }
    };
    let extra_binds: Vec<_> = if schema.config.add_bind_values.is_empty() {
        vec![]
    } else {
        schema.config.add_bind_values.iter()
        .map(|f| {
            let name: syn::Ident = syn::parse_str(&f.name)?;
            Ok(quote! {
                .push_bind(&ctx.#name)
            })
        }).collect::<Result<_, syn::Error>>()?
    };

    let expanded = quote! {
        use sqlx::{Sqlite, QueryBuilder};
        use polars::prelude::*;

        #[derive(Debug, Clone)]
        pub struct #csv_name {
            #(#struct_fields),*
        }

        impl #csv_name {
            pub fn loan_schema() -> SchemaRef {
                Arc::new(Schema::from_iter(vec![
                    #(#load_schemas_fields),*
                ]))
            }
            pub fn optional_fields() -> Arc<Vec<&'static str>> {
                Arc::new(vec![
                    #(#optional_fields),*
                ])
            }
        }

        pub struct #insert_query;

        pub struct #column_names<'a> {
            #(#csv_column),*
        }

        #extra_para_struct

        impl dto_traits::bulk_csv_insert::BulkInsert for #insert_query {
            type Context = #extra_para_type;
            type Columns<'a> = #column_names<'a>;

            fn prepare<'a>(
                df: &'a DataFrame,
            ) -> anyhow::Result<Self::Columns<'a>> {
                Ok(#column_names {
                    #(#prepare_csv_fields),*
                })
            }

            fn insert(
                qb: &mut QueryBuilder<Sqlite>,
                start: usize,
                end: usize,
                cols: &Self::Columns<'_>,
                #extra_para_name: &Self::Context
            ) -> anyhow::Result<()> {
                qb.push_values(start..end, |mut b, row| {
                    b
                    #(#bind_fields)*
                    #(#extra_binds)*
                    ;
                });
                Ok(())
            }
        }
    };

    Ok(TokenStream::from(expanded))
}

// fn check_bind_type(ty: &syn::Type) -> TokenStream {
//     let mut res = quote! {};
//     if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
//         res = match () {
//             _ if path.is_ident("u64") => quote! { as i64 },
//             _ => quote! {}
//         }
//     }
//     res
// }

fn rust_type_to_dtype(ty: &str) -> TokenStream {
    match ty {
        "String" | "str" => quote! { DataType::String },

        "i8" => quote! { DataType::Int8 },
        "i16" => quote! { DataType::Int16 },
        "i32" => quote! { DataType::Int32 },
        "i64" => quote! { DataType::Int64 },

        "u8" => quote! { DataType::UInt8 },
        "u16" => quote! { DataType::UInt16 },
        "u32" => quote! { DataType::UInt32 },
        "u64" => quote! { DataType::UInt64 },

        "f32" => quote! { DataType::Float32 },
        "f64" => quote! { DataType::Float64 },

        "bool" => quote! { DataType::Boolean },

        _ => panic!("unsupported type"),
    }
}

fn rust_type_to_chunked(ty: &str) -> Option<&'static str> {
    match ty {
        "String" => Some("StringChunked"),
        "str" => Some("StringChunked"),

        "i8" => Some("Int8Chunked"),
        "i16" => Some("Int16Chunked"),
        "i32" => Some("Int32Chunked"),
        "i64" => Some("Int64Chunked"),

        "u8" => Some("UInt8Chunked"),
        "u16" => Some("UInt16Chunked"),
        "u32" => Some("UInt32Chunked"),
        "u64" => Some("UInt64Chunked"),

        "f32" => Some("Float32Chunked"),
        "f64" => Some("Float64Chunked"),

        "bool" => Some("BooleanChunked"),

        _ => None,
    }
}

fn rust_type_df_cast(ty: &str) -> Cow<'_, str> {
    Cow::Borrowed(match ty {
        "String" => "str",
        // "u64" => "i64",
        _ => ty
    })
}

/*
simple type ver

fn rust_type_to_chunked(ty: &str) -> Option<&'static str> {
    match ty {
        "String" => Some("StringChunked"),
        "str" => Some("StringChunked"),

        "i8" => Some("Int64Chunked"),
        "i16" => Some("Int64Chunked"),
        "i32" => Some("Int64Chunked"),
        "i64" => Some("Int64Chunked"),

        "u8" => Some("Int64Chunked"),
        "u16" => Some("Int64Chunked"),
        "u32" => Some("Int64Chunked"),
        "u64" => Some("Int64Chunked"),

        "f32" => Some("Float64Chunked"),
        "f64" => Some("Float64Chunked"),

        "bool" => Some("BooleanChunked"),

        _ => None,
    }
}

fn rust_type_df_cast(ty: &str) -> Cow<'_, str> {
    Cow::Borrowed(match ty {
        "str" => "str",
        "String" => "str",
        "i8" => "i64",
        "i16" => "i64",
        "i32" => "i64",
        "i64" => "i64",

        "u8" => "i64",
        "u16" => "i64",
        "u32" => "i64",
        "u64" => "i64",

        "f32" => "f64",
        "f64" => "f64",

        "bool" => "bool",

        _ => ty
    })
}

*/