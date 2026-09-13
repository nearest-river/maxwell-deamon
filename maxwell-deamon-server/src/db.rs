
use uuid::Uuid;
use sqlx::PgPool;
use crate::user::User;
use email_address::EmailAddress;


#[allow(async_fn_in_trait)]
pub trait PoolExt {
  async fn create_new_user(&self,name: &str,email: EmailAddress,password_hash: Box<str>)-> anyhow::Result<User>;
}

impl PoolExt for PgPool {
  async fn create_new_user(&self,name: &str,email: EmailAddress,password_hash: Box<str>)-> anyhow::Result<User> {
    let user=sqlx::query_as!(
      User,
      r#"
      INSERT INTO users (id, name, email, password_hash)
      VALUES ($1, $2, $3, $4)
      RETURNING id, name, email, password_hash, created_at
      "#,
      Uuid::now_v7(),
      &name,
      email.as_str(),
      &password_hash
    )
    .fetch_one(self)
    .await?;

    Ok(user)
  }
}






























