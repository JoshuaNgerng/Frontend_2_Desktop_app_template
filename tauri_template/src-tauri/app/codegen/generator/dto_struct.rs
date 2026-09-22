use std::io::Write;
use std::{collections::{HashMap, HashSet}, fs};
use std::path::Path;
use convert_case::{Casing, Case};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use super::type_inference::{ts_to_rust_type, infer_enum_case};
use super::load::{load_yaml, load_schema_n_dep};
use crate::codegen::SetupConfig;
use crate::codegen::types::{DtoConfig, DtoConfigData, Schema};

pub fn gen_dto_from_db(
    setup: &SetupConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let schemas = load_all_schemas(
        &setup.input_path.dto, &setup.input_path.db_schema
    )?;
    let output_path= format!(
        "{}/{}/{}", setup.output_path.target_dir, setup.output_path.prefix, setup.output_path.dto
    );
    let db_mod= format!("{}/{}", setup.output_path.prefix, setup.output_path.db_path);
    write_all_schemas(schemas, &output_path, &db_mod)?;
    Ok(())
}

fn write_all_schemas(
    schemas: HashMap<String, DtoConfigData>, 
    output_path: &str, db_import_mod: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new(output_path);
    let mut mods = String::new();
    std::fs::create_dir_all(output_dir)?;
    for (fname, schema) in &schemas {
        let tokens = generate_struct(schema, db_import_mod)?;
        let output = prettyplease::unparse(
            &syn::parse_file(&tokens.to_string())?
        );
        let file_path = output_dir.join(format!("{fname}.rs"));
        let mut file = fs::File::create(file_path)?;
        mods = format!("{mods}pub mod {fname};\n");
        file.write(output.as_bytes())?;
    }
    let mod_file_path = output_dir.join("mod.rs");
    let mut mod_file = fs::File::create(mod_file_path)?;
    mod_file.write(mods.as_bytes())?;
    Ok(())
}

fn load_all_schemas(
    dto_path: &str, db_path: &str
) -> Result<HashMap<String, DtoConfigData>, Box<dyn std::error::Error>> {
    let mut res = HashMap::new();
    let schema_dir = Path::new(dto_path);
    let db_dir = Path::new(db_path);
    for entry in fs::read_dir(schema_dir)? {
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
        let config: DtoConfig = load_yaml(path)?;
        let db_fname = Path::new(
            &config.model_target
        ).with_extension("yaml");
        let db_path = db_dir.join(db_fname).with_extension("yaml");
        let data: Schema = if config.load_dep { 
            load_schema_n_dep(&db_path)? 
        } else { 
            load_yaml(&db_path)? 
        };
        res.insert(name, DtoConfigData{
            config,
            data
        });

    }
    Ok(res)
}

fn generate_struct(
    schema: &DtoConfigData, db_import_mod: &str
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let struct_name_ref = &schema.data.table.as_ref()
    .unwrap().to_case(Case::Pascal);
    let struct_name = format_ident!("{}", struct_name_ref);
    let dto_name = format_ident!("{}", format!("{struct_name_ref}Dto"));
    let db_import_mod: syn::Path = syn::parse_str(&db_import_mod.replace("/", "::"))?;
    let db_mod_name: syn::Ident = syn::parse_str(&schema.config.model_target)?;

    let exclude: HashSet<_> = schema.config.exclude.iter().collect();

    let mapping: HashMap<_, _> = schema.config.mapping.iter()
    .map(|v| (v.target.clone(), v.new.clone()))
    .collect();

    let transform = schema.config.convert_case.clone();

    let filtered: Vec<_> =  schema.data.fields.iter()
    .filter(|f| !exclude.contains(&f.name))
    .collect();

    let struct_fields: Vec<_> = filtered.iter()
    .map(|f| {
        let name = format_ident!("{}", f.name);

        let base_ty: syn::Type = syn::parse_str(&ts_to_rust_type(&f.ty))?;
    
        // wrap in Option<T> if nullable
        let ty: syn::Type = if f.nullable {
            syn::parse_quote!(Option<#base_ty>)
        } else {
            base_ty
        };
        let rename = convert_case_helper(
            mapping.get(&f.name)
            .unwrap_or(&f.name),
            transform.as_ref()
        );
        if rename == f.name {
            Ok(quote! {
                pub #name: #ty
            })
        } else {
            Ok(quote! {
                #[serde(rename = #rename)]
                pub #name: #ty
            })
        }

    }).collect::<Result<_, syn::Error>>()?;

    let transfer_fields: Vec<_> = filtered.iter()
    .map(|f| {
        let name = format_ident!("{}", f.name);
        Ok(quote! {
            #name : src.#name
        })
    }).collect::<Result<_, syn::Error>>()?;

    Ok(
        quote! {
            use serde::{Deserialize, Serialize};
            use crate::#db_import_mod::#db_mod_name::#struct_name;

            #[derive(Debug, Clone, Deserialize, Serialize)]
            pub struct #dto_name {
                #(#struct_fields),*
            }

            impl From<#struct_name> for #dto_name {
                fn from(src: #struct_name) -> Self {
                    Self {
                        #(#transfer_fields),*
                    }
                }
            }
        }
    )
}

fn convert_case_helper(ref_: &String, case: Option<&String>) -> String {
    let check_case = match case {
        Some(v) => v,
        None => { return ref_.to_string(); },
    };
    let fin = match infer_enum_case(check_case) {
        Some(c) => ref_.to_case(c),
        None => { return ref_.to_string(); }
    };
    fin
}
