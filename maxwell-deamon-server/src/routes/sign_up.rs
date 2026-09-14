
use std::str::FromStr;
use super::prelude::*;
use email_address::EmailAddress;

use maxwell_deamon_server::auth::{
  self,
  SignUpForm,
};


#[inline(always)]
#[axum::debug_handler]
pub async fn sign_up(State(state): State<AppState>,body: Bytes)-> Result<impl IntoResponse,String> {
  _sign_up(state,body).await.map_err(|err| err.to_string())
}

async fn _sign_up(AppState { db, email_handler }: AppState,body: Bytes)-> anyhow::Result<impl IntoResponse> {
  let SignUpForm { name, email, password_hash }=rmp_serde::from_slice::<SignUpForm>(&body)?;
  let email=EmailAddress::from_str(&email)?;

  if db.email_exists(email.as_str()).await? {
    return Ok(());
  }

  let (token,token_hash)=auth::generate_verification_token();

  let user=db.create_new_user(&name,&email,password_hash).await?;

  db.register_verification_token(user.id,&token_hash).await?;
  email_handler.send_verification_email(&email,BASE_URL,&token).await?;
  Ok(())
}




