use std::sync::Arc;
use axum::{extract::{Path, State}, http::StatusCode, response::{IntoResponse, Redirect}, Json};
use crate::{db::AppState, users::{user::User, user_mgmt::{add_user, get_user, get_users, update_user}}};

pub async fn get_users_handler(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match get_users(Arc::clone(&state.db)).await {
        Ok(users) => Json(users).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_user_handler(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    match get_user(Arc::clone(&state.db), &username).await {
        Ok(user) => {
            if let Some(found_user) = user {
                Json(found_user).into_response()
            } else {
                StatusCode::NOT_FOUND.into_response()
            }
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// test comment
pub async fn add_user_handler(
    State(state): State<Arc<AppState>>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    match add_user(Arc::clone(&state.db), &user).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            eprintln!("Error adding user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_user_handler(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    match update_user(Arc::clone(&state.db), &user).await {
        Ok(user_response) => {
            if username != user_response.username {
                Redirect::to(format!("/users/{}", user_response.username).as_str()).into_response()
            } else {
                Json(user_response).into_response()
            }
        }
        Err(e) => {
            eprintln!("Error updating user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
