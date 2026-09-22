mod codegen;

use codegen::SetupConfig;
use codegen::generator::load_yaml;
use codegen::generator::db_struct::make_yaml_db_struct;
use codegen::generator::dto_struct::gen_dto_from_db;
use codegen::generator::impl_csv::make_csv_ingestion;
use codegen::generator::schemas_struct::gen_schemas;

use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>>   {
    println!("cargo:rerun-if-changed=config");
    let setup: SetupConfig = load_yaml("config/build_setup.yaml")?;
    std::fs::create_dir_all(&setup.output_path.target_dir)?;
    make_yaml_db_struct(&setup)?;
    gen_dto_from_db(&setup)?;
    make_csv_ingestion(&setup)?;
    gen_schemas(&setup)?;
    write_gen_mod(&setup)?;
    tauri_build::build();
    Ok(())
}

fn write_gen_mod(setup: &SetupConfig) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!(
        "{}/{}/{}",
        &setup.output_path.target_dir, &setup.output_path.prefix, &"mod.rs"
    ); 

    let output_path = std::path::Path::new(&path);
    let mut mod_file = std::fs::File::create(output_path)?;
    let ref_mod = "pub mod dto;\npub mod models;\npub mod schemas;";
    mod_file.write(ref_mod.as_bytes())?;
    Ok(())
}