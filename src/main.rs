mod config;
mod middleware;
mod handlers;
mod models;
pub mod services;
pub mod repositories;

use axum::routing::get;
use axum::{Router, middleware::from_fn};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() {
    dotenv().ok();

    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let console_layer = tracing_subscriber::fmt::layer().with_writer(std::io::stdout);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(console_layer)
        .with(file_layer)
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

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .layer(from_fn(middleware::logging::log_requests))
        .with_state(pool);

    let addr = format!("{}:{}", db_config.host, db_config.port);
    tracing::info!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
