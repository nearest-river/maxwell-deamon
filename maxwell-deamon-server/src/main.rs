
use sqlx::PgPool;
use tracing::info;
use resend_rs::Resend;
use zeroize::Zeroizing;
use tokio::net::TcpListener;
use email_address::EmailAddress;
use maxwell_deamon_serde::SignUpForm;

use sha2::{
  Sha256,
  Digest,
};

use maxwell_deamon_server::{
  db::DatabasePool,
  email::EmailHandler,
  verify::{
    self,
    LoginQuery,
    VerificationQuery,
    VerificationError,
  },
};

use std::{
  env,
  str::FromStr,
};

use axum::{
  Router,
  body::Bytes,
  routing::{
    get,
    post,
  },
  extract::{
    Query,
    State,
  },
  response::{
    Redirect,
    IntoResponse,
  },
};


#[derive(Clone)]
struct AppState {
  db: DatabasePool,
  email_handler: EmailHandler,
}

static BASE_URL: &str="http://127.0.0.1:3000";

#[tokio::main]
async fn main()-> anyhow::Result<()> {
  let state=AppState::init().await?;

  let router=Router::new()
  .route("/",get(home))
  .route("/hello",get(hello))
  .route("/sign_up",post(sign_up))
  .route("/verify",get(verify))
  .route("/login",get(login))
  .with_state(state);

  let listener=TcpListener::bind(BASE_URL).await?;
  info!("listening on http://{}",listener.local_addr()?);


  axum::serve(listener,router).await?;
  Ok(())
}

async fn hello()-> impl IntoResponse {
  "hello world"
}

async fn home()-> impl IntoResponse {
  "todo"
}

#[inline(always)]
#[axum::debug_handler]
async fn sign_up(State(state): State<AppState>,body: Bytes)-> Result<impl IntoResponse,String> {
  _sign_up(state,body).await.map_err(|err| err.to_string())
}

async fn _sign_up(AppState { db, email_handler }: AppState,body: Bytes)-> anyhow::Result<impl IntoResponse> {
  let SignUpForm { name, email, password_hash }=rmp_serde::from_slice::<SignUpForm>(&body)?;
  let email=EmailAddress::from_str(&email)?;

  if db.email_exists(email.as_str()).await? {
    return Ok(());
  }

  let (token,token_hash)=verify::generate_verification_token();

  let user=db.create_new_user(&name,&email,password_hash).await?;

  db.register_verification_token(user.id,&token_hash).await?;
  email_handler.send_verification_email(&email,BASE_URL,&token).await?;
  Ok(())
}

#[inline(always)]
#[axum::debug_handler]
async fn verify(State(AppState { db, .. }): State<AppState>,Query(query): Query<VerificationQuery>)-> Result<Redirect,VerificationError> {
  let token_hash=Sha256::digest(query.token.as_bytes());

  db.verify(&token_hash).await?;

  Ok(Redirect::to("/login?verified=true"))
}

async fn login(State(AppState { db, .. }): State<AppState>,Query(query): Query<LoginQuery>)-> impl IntoResponse {
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



