use std::{fs, path::Path};
use serde::de::DeserializeOwned;

use crate::codegen::types::Schema;

pub fn load_yaml<P, T>(path: P) -> Result<T, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let content = fs::read_to_string(path)?;
    let value = serde_yaml::from_str::<T>(&content)?;
    Ok(value)
}

pub fn load_schema_n_dep<P>(path: P) -> Result<Schema, Box<dyn std::error::Error>>
where 
    P: AsRef<Path>
{
    // println!("check {}. path");
    let mut res: Schema = load_yaml(&path)?;
    let dir = path.as_ref().parent().unwrap_or(&Path::new("."));

    for f in &res.shared_fields {
        let file_path = dir.join(f).with_extension("yaml");
        let mut base_schema: Schema = load_yaml(&file_path)?;
        res.fields.append(&mut base_schema.fields);
    }
    Ok(res)
} 
