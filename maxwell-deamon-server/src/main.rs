
use std::env;
use sqlx::PgPool;
use zeroize::Zeroizing;
use maxwell_deamon_server::db::PoolExt;



struct AppState {
  pool: PgPool,
}



#[tokio::main]
async fn main()-> anyhow::Result<()> {
  let state=AppState::init().await?;


  Ok(())
}



impl AppState {
  async fn init()-> anyhow::Result<Self> {
    tracing_subscriber::fmt()
    .init();

    let db_url=Zeroizing::new(env::var("DATABASE_URL")?);
    let pool=PgPool::connect(&db_url).await?;
    drop(db_url);

    Ok(Self {
      pool,
    })
  }
}



