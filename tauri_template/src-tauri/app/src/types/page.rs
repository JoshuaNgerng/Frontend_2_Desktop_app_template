pub struct PaginationResult<T> {
    pub data: Vec<T>,
    pub count: u64
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct FilterOptions {
    pub field: String,
    pub value: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DistItems {
    pub dist_type: String,
    pub label: String,
    pub cnt: u64,
}
