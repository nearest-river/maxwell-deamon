

mod auth;
mod pages;
pub(in self) mod prelude;

use crate::AppState;
use tower_http::services::ServeDir;

use axum::{
  Router,
  routing::{
    get,
    post,
  }
};



pub fn router(state: AppState)-> Router {
  let serve_dir=ServeDir::new("static");

  Router::new()
  .merge(auth_pages())
  .merge(public_pages())
  .nest_service("/static",serve_dir)
  .with_state(state)
}

fn public_pages()-> Router<AppState> {
  Router::new()
  .route("/",get(pages::home))
  .route("/hello",get(pages::hello))
}

fn auth_pages()-> Router<AppState> {
  Router::new()
  .route("/sign_up",post(auth::sign_up))
  .route("/verify",post(auth::verify))
  .route("/login",post(auth::login))
}



















