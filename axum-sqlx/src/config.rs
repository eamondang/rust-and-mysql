use serde::Deserialize;

use crate::{error::Error, Result};

#[derive(Deserialize)]
pub struct PostgreSQL {
  pub dsn: String,
  pub maxcons: u32,
}

#[derive(Deserialize)]
pub struct WebConfig {
  pub addr: String,
}

#[derive(Deserialize)]
pub struct Config {
  pub postgresql: PostgreSQL,
  pub web: WebConfig,
}

impl Config {
  pub fn from_env() -> Result<Self> {
    config::Config::builder()
      .add_source(config::Environment::default())
      .build()
      .map_err(Error::from)?
      .try_deserialize()
      .map_err(Error::from)
  }
}
