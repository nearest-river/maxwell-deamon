
mod routes;

use std::env;
use sqlx::PgPool;
use tracing::info;
use resend_rs::Resend;
use zeroize::Zeroizing;
use tokio::net::TcpListener;

use maxwell_deamon_server::{
  db::DatabasePool,
  email::EmailHandler,
};

use axum::{
  Router,
  routing::{
    get,
    post,
  },
};


#[derive(Clone)]
pub struct AppState {
  pub db: DatabasePool,
  pub email_handler: EmailHandler,
}

pub static BASE_ADDR: &str="127.0.0.1:3000";
pub static BASE_URL: &str="http://127.0.0.1:3000";

#[tokio::main]
async fn main()-> anyhow::Result<()> {
  let state=AppState::init().await?;

  let router=Router::new()
  .route("/",get(routes::home))
  .route("/hello",get(routes::hello))
  .route("/sign_up",post(routes::sign_up))
  .route("/verify",get(routes::verify))
  .route("/login",get(routes::login))
  .with_state(state);

  let listener=TcpListener::bind(BASE_ADDR).await?;
  info!("listening on http://{}",listener.local_addr()?);


  axum::serve(listener,router).await?;
  Ok(())
}




impl AppState {
  async fn init()-> anyhow::Result<Self> {
    tracing_subscriber::fmt()
    .init();
    dotenvy::dotenv()?;

    let db_url=Zeroizing::new(env::var("DATABASE_URL")?);
    let db=DatabasePool::new(PgPool::connect(&db_url).await?);

    let resend_api_key=Zeroizing::new(env::var("RESEND_API_KEY")?);
    let email_handler=EmailHandler::new(Resend::new(&resend_api_key));

    Ok(Self {
      db,
      email_handler,
    })
  }
}



