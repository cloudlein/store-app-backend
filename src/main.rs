mod config;

use axum::Router;
use axum::routing::get;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let db_config = config::db::DbConfig::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_config.database_url)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    let addr =format!("{}:{}", db_config.host, db_config.port);
    tracing::info!("Server running at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

}