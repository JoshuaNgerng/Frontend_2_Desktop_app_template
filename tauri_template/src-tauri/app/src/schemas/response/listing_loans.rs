use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all="camelCase")]
pub struct OffsetPaginatedResponse {
    pub data:  Vec<>, // put data class here
    pub total: u32, 
    pub page:  u32,
    pub limit: u32,
    pub total_pages: u32
}

