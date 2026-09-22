use serde::Serialize;

#[derive(Serialize)]
pub struct BulkUploadResult {
    pub no_inserted: i32,
    pub batch: String,
}

#[derive(Serialize)]
pub struct BulkUploadFileResult {
    #[serde(flatten)]
    pub base: BulkUploadResult,
    pub filename: String,
}