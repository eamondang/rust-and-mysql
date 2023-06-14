use error::Error;
use std::sync::Arc;

pub mod config;
pub mod error;
pub mod model;

pub type Result<T> = std::result::Result<T, Error>;

pub struct AppState {
  pub pool: Arc<sqlx::PgPool>,
}
