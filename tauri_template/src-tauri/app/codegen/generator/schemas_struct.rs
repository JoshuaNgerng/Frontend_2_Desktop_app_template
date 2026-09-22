use std::io::Write;
use std::fs;
use std::path::Path;
use convert_case::{Casing, Case};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use super::type_inference::{ts_to_rust_type};
use super::load::{load_yaml};
use crate::codegen::SetupConfig;
use crate::codegen::types::{SchemaEx, SchemasConfig};

pub fn gen_schemas(
    setup: &SetupConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let schemas: SchemasConfig = load_yaml(&setup.input_path.schemas)?;
    let output_path= format!(
        "{}/{}/{}", setup.output_path.target_dir, setup.output_path.prefix, setup.output_path.schemas
    );
    write_all_schemas(schemas, &output_path)?;
    Ok(())
}

fn write_all_schemas(
    schemas: SchemasConfig, output_path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new(output_path);
    write_schemas(&schemas.request, output_dir, "request")?;
    write_schemas(&schemas.response, output_dir, "response")?;
    let file_path = output_dir.join("mod.rs");
    let mut file = fs::File::create(file_path)?;
    file.write(format!("pub mod request;\npub mod response;\n").as_bytes())?;
    Ok(())
}

fn write_schemas(
    schemas: &Vec<SchemaEx>, output_path: &Path, name: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let mut fin_stream = quote! {use serde::{Deserialize, Serialize};};
    for schema in schemas {
        println!("debug {}", schema.schema_name );
        let struct_section = generate_struct(schema)?;
        fin_stream.extend(struct_section);
    }
    std::fs::create_dir_all(output_path)?;
    let file_path = output_path.join(format!("{name}.rs"));
    let mut file = fs::File::create(file_path)?;
    let output = prettyplease::unparse(
        &syn::parse_file(&fin_stream.to_string())?
    );
    file.write(output.as_bytes())?;
    Ok(())
}

fn generate_struct(
    schema: &SchemaEx
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let struct_name = format_ident!("{}", schema.schema_name);


    let struct_fields: Vec<_> = schema.fields.iter()
    .map(|f| {
        let name =  syn::LitStr::new(&f.name, proc_macro2::Span::call_site());
        let rename = format_ident!("{}", &f.name.to_case(Case::Snake));
        let base_ty: syn::Type = syn::parse_str(&ts_to_rust_type(&f.ty))?;
    
        // wrap in Option<T> if nullable
        let ty: syn::Type = if f.nullable {
            syn::parse_quote!(Option<#base_ty>)
        } else {
            base_ty
        };
        let rename_token = if rename == f.name {
            quote! {}
        } else {
            quote! {
                #[serde(
                    rename(serialize = #name),
                    rename(deserialize = #name)
                )]
            }
        };
        let enable_default_token = if f.default {
            quote! {#[serde(default)]}
        } else {
            quote! {}
        };

        Ok(
            quote! {
                #rename_token
                #enable_default_token
                pub #rename: #ty
            }
        )
        // if rename == f.name {
        //     Ok(quote! {
        //         pub #rename: #ty
        //     })
        // } else {
        //     Ok(quote! {
        //         #[serde(
        //             rename(serialize = #name),
        //             rename(deserialize = #name)
        //         )]
        //         pub #rename: #ty
        //     })
        // }

    }).collect::<Result<_, syn::Error>>()?;

    Ok(
        quote! {
            #[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
            pub struct #struct_name {
                #(#struct_fields),*
            }

        }
    )
}
