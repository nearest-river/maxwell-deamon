

use axum::{
  http::StatusCode,
  response::{
    Response,
    IntoResponse,
  },
};


#[derive(serde::Deserialize,serde::Serialize)]
pub struct SignUpForm {
  pub name: Box<str>,
  pub email: Box<str>,
  pub password_hash: Box<str>,
}

#[derive(serde::Deserialize)]
pub struct VerificationQuery {
  pub token: String,
}

#[derive(serde::Deserialize)]
pub struct LoginQuery {
  pub verified: bool
}


#[derive(Debug,thiserror::Error)]
pub enum VerificationError {
  #[error("invalid or expired verification token")]
  InvalidToken,
  #[error("database error")]
  Database(#[from] sqlx::Error),
}

impl IntoResponse for VerificationError {
  #[inline(always)]
  fn into_response(self)-> Response {
    match self {
      Self::InvalidToken=> (StatusCode::BAD_REQUEST,"Invalid or expired verification token").into_response(),
      Self::Database(err)=> {
        tracing::error!("database error during email verification: {err}");
        (StatusCode::INTERNAL_SERVER_ERROR,"Something went wrong").into_response()
      }
    }
  }
}


