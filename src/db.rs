extern crate tokio;
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Client>,
}

pub async fn connect() -> Result<Client, tokio_postgres::Error> {
    let (client, connection) =
        tokio_postgres::connect(
            "host=localhost user=anirudh",
            NoTls
        ).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}
