use std::sync::Arc;
use axum::{routing::{get, post, put}, Router};
use crate::{
    db::{self, AppState},
    routes::user_routes::{add_user_handler, get_user_handler, get_users_handler, update_user_handler}
};

pub async fn run_server() -> Result<(), tokio_postgres::Error> {
    let db = db::connect().await.unwrap();
    let db = Arc::new(db);
    let state = Arc::new(AppState { db });

    let app = Router::new()
        .route("/", get(|| async { "A Postgres Demo backend powered by tokio and axum!" }))
        .route("/users", get(get_users_handler))
        .route("/users/{username}", get(get_user_handler))
        .route("/users", post(add_user_handler))
        .route("/users/{username}", put(update_user_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    println!("Server is running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();

    Ok(())
}
