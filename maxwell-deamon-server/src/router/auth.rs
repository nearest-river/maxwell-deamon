
use rand::Rng;
use super::prelude::*;
use std::str::FromStr;
use email_address::EmailAddress;
use maxwell_deamon_server::auth::*;

use sha2::{
  Digest,
  Sha256,
};

use axum::{
  response::{
    Redirect,
    IntoResponse,
  },
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

  let (token,token_hash)=generate_verification_token();

  let user=db.create_new_user(&name,&email,password_hash).await?;

  db.register_verification_token(user.id,&token_hash).await?;
  email_handler.send_verification_email(&email,BASE_URL,&token).await?;
  Ok(())
}


#[axum::debug_handler]
pub async fn verify(State(AppState { db, .. }): State<AppState>,Query(query): Query<VerificationQuery>)-> Result<Redirect,VerificationError> {
  let token_hash=Sha256::digest(query.token.as_bytes());

  db.verify(&token_hash).await?;

  Ok(Redirect::to("/login?verified=true"))
}


pub async fn login(State(AppState { db: _db, .. }): State<AppState>,Query(_query): Query<LoginQuery>)-> impl IntoResponse {
}


















fn generate_verification_token()-> (String,Vec<u8>) {
  let mut bytes=[0u8;32];

  rand::rng()
  .fill_bytes(&mut bytes);

  let token=hex::encode(bytes);

  let hash=Sha256::digest(bytes);

  (token,hash.to_vec())
}


