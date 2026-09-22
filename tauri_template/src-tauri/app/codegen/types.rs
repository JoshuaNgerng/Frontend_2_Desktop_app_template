use serde::Deserialize;


#[derive(Clone, Debug, Deserialize)]
pub struct SetupConfig {
    pub input_path: InputConfig,
    pub output_path: OutputConfig
}

#[derive(Clone, Debug, Deserialize)]
pub struct InputConfig {
    pub db_schema: String,
    pub dto: String,
    pub csv: String,
    pub schemas: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct OutputConfig {
    pub target_dir: String,
    pub prefix: String,
    pub db_path: String,
    pub dto: String,
    pub schemas: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct Schema {
    #[serde(default)]
    pub table: Option<String>,
    #[serde(default)]
    pub shared_fields: Vec<String>,
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(default = "default_false")]
    pub nullable: bool
}

pub struct DtoConfigData {
    pub config: DtoConfig,
    pub data: Schema
}

#[derive(Clone, Debug, Deserialize)]
pub struct DtoConfig {
    #[serde(rename = "dto_target")]
    pub model_target: String,
    #[serde(default)]
    pub mapping: Vec<DtoFieldMapping>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub convert_case: Option<String>,
    #[serde(default = "default_false")]
    pub load_dep: bool
}

#[derive(Clone, Debug, Deserialize)]
pub struct DtoFieldMapping {
    pub target: String,
    pub new: String
}

pub struct Csv2DB {
    pub config: Csv2DBConfig,
    pub data: Schema
}

#[derive(Clone, Debug, Deserialize)]
pub struct Csv2DBConfig {
    pub target: String,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub optional_fields: Vec<String>,
    #[serde(default)]
    pub add_bind_values: Vec<Field>
}

#[derive(Clone, Debug, Deserialize)]
pub struct FieldWithDefault {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(default = "default_false")]
    pub nullable: bool,
    #[serde(default = "default_false")]
    pub default: bool
}

#[derive(Clone, Debug, Deserialize)]
pub struct SchemaEx {
    pub schema_name: String,
    #[serde(default)]
    pub fields: Vec<FieldWithDefault>
}

#[derive(Clone, Debug, Deserialize)]
pub struct SchemasConfig {
    #[serde(default)]
    pub request: Vec<SchemaEx>,
    #[serde(default)]
    pub response: Vec<SchemaEx>
}

// fn default_true() -> bool {
//     true
// }

fn default_false() -> bool {
    false
}
