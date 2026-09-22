use std::io::Write;
use std::{collections::HashMap, fs};
use std::path::Path;
use convert_case::{Casing, Case};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use super::type_inference::ts_to_rust_type;
use super::load::load_yaml;
use crate::codegen::SetupConfig;
use crate::codegen::types::Schema;

pub fn make_yaml_db_struct(setup: &SetupConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut schemas = load_all_schemas(&setup.input_path.db_schema)?;

    resolve_dependencies(&mut schemas);

    let output_path= format!(
        "{}/{}/{}", setup.output_path.target_dir, setup.output_path.prefix, setup.output_path.db_path
    );
    write_all_schemas(schemas, &output_path)?;

    Ok(())
}

fn resolve_dependencies(schemas: &mut HashMap<String, Schema>) {
    let dep_schemas: HashMap<_, _> = schemas.iter().map(
        |(k, v)| (k.clone(), v.shared_fields.clone())
    ).collect();

    let mut find_shared_fields = HashMap::new();
    for (fname, deps) in dep_schemas {
        let mut shared_fields = Vec::new();
        for dep in deps {
            let ref_s =  match schemas.get(&dep) {
                Some(s) => s,
                None => continue, 
            };
            shared_fields.extend(ref_s.fields.iter().cloned());
        }
        find_shared_fields.insert(fname, shared_fields);
    }

    for (fname, shared_fields) in &mut find_shared_fields {
        let schema= match  schemas.get_mut(fname) {
            Some(s) => s,
            None => continue,
        };
        schema.fields.append(shared_fields);
    }

}

fn write_all_schemas(
    schemas: HashMap<String, Schema>, path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new(path);
    let mut mods = String::new();
    std::fs::create_dir_all(output_dir)?;
    for (fname, schema) in &schemas {
        if schema.table.is_none() { continue; }
        let tokens = generate_struct(schema)?;
        let output = prettyplease::unparse(
            &syn::parse_file(&tokens.to_string())?
        );
        let file_path = output_dir.join(format!("{fname}.rs"));
        let mut file = fs::File::create(file_path)?;
        file.write(output.as_bytes())?;
        mods = format!("{mods}pub mod {fname};\n");
    }
    let mod_file_path = output_dir.join("mod.rs");
    let mut mod_file = fs::File::create(mod_file_path)?;
    mod_file.write(mods.as_bytes())?;
    Ok(())
}

fn load_all_schemas(
    path: &str
) -> Result<HashMap<String, Schema>, Box<dyn std::error::Error>> {
    let mut res = HashMap::new();
    let schema_dir = Path::new(path);
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
        res.insert(name, load_yaml(path)?);
    }
    Ok(res)
}

fn generate_struct(
    schema: &Schema
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let struct_name = format_ident!(
        "{}", schema.table.as_ref().unwrap().to_case(Case::Pascal)
    );
    let fields: Vec<_> = schema.fields.iter().map(|f| {
        let name = format_ident!("{}", f.name);
        let base_ty: syn::Type = syn::parse_str(&ts_to_rust_type(&f.ty))?;
    
        // wrap in Option<T> if nullable
        let ty: syn::Type = if f.nullable {
            syn::parse_quote!(Option<#base_ty>)
        } else {
            base_ty
        };
    
        Ok(quote! {
            pub #name: #ty
        })
    }).collect::<Result<_, syn::Error>>()?;

    Ok(
        quote! {
            #[derive(Debug, Clone, sqlx::FromRow)]
            pub struct #struct_name {
                #(#fields),*
            }
        }
    )
}
