// Standardized response format
#[derive(Serialize)]
pub struct PaginateResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub limit: u32,
    pub total: i64,
}


#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>
}

impl PaginationQuery {
    pub fn apply_filters<'args>(
        &'args self,
        builder: &mut QueryBuilder<'args, Postgress>
    ) {
        if let Some(search) = &self.search {
            builder.push("title ILIKE ");
            let search_term = format!("%{}%", search);
            builder.push_bind(search_term);
        }
    }
}