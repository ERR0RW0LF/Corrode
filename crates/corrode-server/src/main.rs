use dotenvy::dotenv;
use sqlx::{PgPool, Pool, Postgres};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use anyhow::Result;

#[cfg(test)]
mod tests;
mod auth;
mod db;


#[tokio::main]
async fn main() -> Result<()>{
    dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| {
                "axum_login=debug,tower_sessions=debug,sqlx=warn,tower_http=debug,\
                    corrode_server=info"
                    .into()
            },
        )))
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    let pool_address = if let Ok(a) = std::env::var("DATABASE_URL") {
        a
    } else {
        panic!()
    };

    if let Ok(p) = std::env::var("PASSWORD_PEPPER") {
        println!("{}",p);
    };

    let pool = db::setup_pool(&pool_address).await?;
 

    pool.close().await;
    Ok(())
}
