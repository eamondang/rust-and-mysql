use std::sync::Arc;

use axum::{routing, Extension, Router, Server};
use axum_sqlx::{config::Config, AppState};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
  dotenv().ok();
  tracing_subscriber::fmt::init();

  let cfg = Config::from_env().map_err(|e| tracing::error!("Error loading config: {}", e)).unwrap();
  let pool = sqlx::postgres::PgPoolOptions::new()
    .max_connections(cfg.postgresql.maxcons)
    .connect(&cfg.postgresql.dsn)
    .await
    .map_err(|e| tracing::error!("Error connecting to database: {}", e))
    .unwrap();

  let app = Router::new()
    .layer(Extension(Arc::new(AppState { pool: Arc::new(pool) })))
    .route("/", routing::get(|| async { "Hello World!!" }));

  tracing::info!("Connected Database");
  tracing::info!("Server running on Port: {}", &cfg.web.addr);

  let addr = &cfg.web.addr;
  let _ = Server::bind(&addr.parse().unwrap()).serve(app.into_make_service()).await;
}
