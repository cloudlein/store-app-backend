use axum::extract::{Query, State};
use sqlx::query::Query;
use crate::models::response::PaginationQuery;

// Product handlers will go here
pub async fn get_products(
    Query(params): Query<PaginationQuery>,
    State(state): State<>
)