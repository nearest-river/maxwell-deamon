
use super::prelude::*;

use sha2::{
  Digest,
  Sha256,
};

use maxwell_deamon_server::auth::{
  VerificationError,
  VerificationQuery,
};



#[inline(always)]
#[axum::debug_handler]
pub async fn verify(State(AppState { db, .. }): State<AppState>,Query(query): Query<VerificationQuery>)-> Result<Redirect,VerificationError> {
  let token_hash=Sha256::digest(query.token.as_bytes());

  db.verify(&token_hash).await?;

  Ok(Redirect::to("/login?verified=true"))
}





