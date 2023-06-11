use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Default, Debug, Deserialize, Serialize, sqlx::Type, Clone)]
#[repr(u8)]
pub enum MemberTypes {
  #[default]
  Normal,
  Silver,
  Gold,
  Diamond,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Member {
  pub id: i32,
  pub name: String,
  pub dateline: chrono::DateTime<chrono::Local>,
  pub balance: u32,
  pub types: MemberTypes,
  pub is_del: bool,
}
