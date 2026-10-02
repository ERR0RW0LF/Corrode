use sqlx::{PgPool, Pool, Postgres};
use anyhow::{Ok, Result};


pub async fn setup_pool(url: &str) -> Result<Pool<Postgres>> {
    let pool = PgPool::connect(url).await?;

    sqlx::migrate!()
        .run(&pool)
        .await?;

    Ok(pool)
}