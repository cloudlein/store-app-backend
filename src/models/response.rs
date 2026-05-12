// Standardized response format
#[derive(Serialize)]
pub struct PaginateResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub limit: u32,
    pub total: i64,
}
