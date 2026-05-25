use sqlx::PgPool;

pub struct SqlxTodoRepository {
    pool: PgPool,
}

impl SqlxTodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}