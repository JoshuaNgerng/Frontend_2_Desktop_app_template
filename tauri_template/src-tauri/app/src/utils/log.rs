use serde::Serialize;
use std::fs;

pub fn log_json<T: Serialize>(value: &T, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    fs::write(path, json)?;
    Ok(())
}
