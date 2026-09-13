
use uuid::Uuid;
use chrono::{
  Utc,
  DateTime,
};



#[derive(serde::Serialize,serde::Deserialize)]
pub struct User {
  pub id: Uuid,
  pub name: Box<str>,
  pub email: Box<str>,
  pub password_hash: Box<str>,
  pub created_at: DateTime<Utc>,
}





