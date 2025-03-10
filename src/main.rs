use std::sync::Arc;
use axum::{routing::get, Router};
use pg::{db::{self, AppState}, routes::user_routes::get_users_handler};

#[tokio::main]
async fn main() -> Result<(), tokio_postgres::Error> {
    let db = db::connect().await?;
    let db = Arc::new(db);
    let state = Arc::new(AppState {
        db
    });

    let app = Router::new()
        .route("/", get(|| async {"Hello, World!"}))
        .route("/users", get(get_users_handler))
        .with_state(state);

    let listener =
        tokio::net::TcpListener::bind("localhost:3000")
            .await
            .unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();

    Ok(())
}
