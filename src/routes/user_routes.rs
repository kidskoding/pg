use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use crate::{db::AppState, users::user_mgmt::get_users};

pub async fn get_users_handler(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match get_users(Arc::clone(&state.db)).await {
        Ok(users) => Json(users).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
