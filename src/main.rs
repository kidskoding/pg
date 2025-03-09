pub mod users;

use axum::{routing::get, Router};
use pg::db;

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error> {
    let app = Router::new()
        .route("/", get(|| async {"Hello, World!"}));
    let db = db::connect().await?;

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();

    Ok(())
}
