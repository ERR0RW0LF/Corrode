use axum_login::AuthnBackend;
use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use anyhow::Result;
use uuid::uuid;

use crate::auth::Backend;

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

    let pool = db::setup_pool(&pool_address).await?;


    let pepper = std::env::var("PASSWORD_PEPPER").unwrap();

    let backend = Backend::new(pool.clone(), pepper);

    println!("{:#?}",backend.get_user(&uuid!("f450bff9-de5c-4121-b7d3-7645ac77a5c3")).await);
    println!("{:#?}",backend.get_user(&uuid!("f450bff9-de5c-4121-b7d3-7645ac77a5c2")).await);
    pool.close().await;
    println!("{:#?}",backend.get_user(&uuid!("f450bff9-de5c-4121-b7d3-7645ac77a5c3")).await);
    Ok(())
}
