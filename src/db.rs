extern crate tokio;
use std::{env, sync::Arc};
use color_eyre::eyre::Result;
use dotenv::dotenv;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
}

pub async fn connect() -> Result<PgPool> {
    dotenv().ok();
    let url = env::var("DB_URL")
        .expect("could not load the DB_URL in .env!");

    let conn = PgPool::connect(&url)
        .await?;

    Ok(conn)
}
