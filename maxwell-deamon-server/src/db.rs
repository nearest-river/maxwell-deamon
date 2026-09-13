
use uuid::Uuid;
use sqlx::PgPool;
use email_address::EmailAddress;

use crate::{
  user::User,
  verify::VerificationError,
};


#[derive(Clone)]
pub struct DatabasePool(PgPool);


impl DatabasePool {
  #[inline(always)]
  pub fn new(pool: PgPool)-> Self {
    Self(pool)
  }

  pub async fn create_new_user(&self,name: &str,email: &EmailAddress,password_hash: Box<str>)-> sqlx::Result<User> {
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
    .fetch_one(&self.0)
    .await?;

    Ok(user)
  }

  pub async fn email_exists(&self,email: &str)-> sqlx::Result<bool> {
    let exists: bool=sqlx::query_scalar!(
      r#"
      SELECT EXISTS(
        SELECT 1
        FROM users
        WHERE email = $1
      )
      "#,
      email
    )
    .fetch_one(&self.0)
    .await?
    .unwrap_or_default();

    Ok(exists)
  }

  pub async fn register_verification_token(&self,user_id: Uuid,token_hash: &[u8])-> sqlx::Result<()> {
    sqlx::query!(
      r#"
      INSERT INTO email_verification_tokens
        (user_id, token_hash, expires_at)
      VALUES
        ($1, $2, NOW() + INTERVAL '5 minutes')
      "#,
      user_id,
      token_hash,
    )
    .execute(&self.0)
    .await?;

    Ok(())
  }

  pub async fn verify(&self,token_hash: &[u8])-> Result<(),VerificationError> {
    let mut tx=self.0.begin().await?;

    let row=sqlx::query!(
      r#"SELECT user_id FROM email_verification_tokens WHERE token_hash = $1 AND expires_at > NOW()"#,
      token_hash,
    )
    .fetch_optional(&mut *tx)
    .await?;

    let Some(row)=row else {
      return Err(VerificationError::InvalidToken);
    };

    sqlx::query!(r#"UPDATE users SET email_verified = TRUE WHERE id=$1"#,row.user_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query!(r#"DELETE FROM email_verification_tokens WHERE user_id = $1"#,row.user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
  }

}






























