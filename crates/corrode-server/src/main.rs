
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use anyhow::Result;

#[cfg(test)]
mod tests;
mod auth;



fn main() -> Result<()>{
    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| {
                "axum_login=debug,tower_sessions=debug,sqlx=warn,tower_http=debug,\
                    corrode-server=info"
                    .into()
            },
        )))
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;


    Ok(())
}
