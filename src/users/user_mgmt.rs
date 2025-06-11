use std::sync::Arc;
use color_eyre::eyre::Result;
use sha2::{Sha256, Digest};
use sqlx::PgPool;
use super::{user::User, user_response::UserResponse};

pub async fn add_user(db: Arc<PgPool>, user: &User) -> Result<()> {
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(user.password.as_bytes());
    let hashed_password = hex::encode(hasher.finalize());

    sqlx::query("INSERT INTO users (username, email, hashed_password) VALUES ($1, $2, $3)")
        .bind(&user.username)
        .bind(&user.email)
        .bind(&hashed_password)
        .execute(db.as_ref())
        .await?;

    Ok(())
}

pub async fn get_user(db: Arc<PgPool>, username: &str) -> Result<Option<UserResponse>> {
    let user: Option<UserResponse> = sqlx::query_as(
        "SELECT username, email FROM users WHERE username = $1"
    )
        .bind(username)
        .fetch_optional(db.as_ref())
        .await?;
    
    Ok(user)
}

pub async fn get_users(db: Arc<PgPool>) -> Result<Vec<UserResponse>> {
    let users: Vec<UserResponse> = sqlx::query_as(
        "SELECT username, email FROM users"
    )
        .fetch_all(db.as_ref())
        .await?;

    Ok(users)
}

pub async fn update_user(db: Arc<PgPool>, user: &User) -> Result<UserResponse> {
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(user.password.as_bytes());
    let hashed_password = hex::encode(hasher.finalize());

    sqlx::query(
        "UPDATE users SET email = $1, hashed_password = $2 WHERE username = $3"
    )
        .bind(&user.email)
        .bind(&hashed_password)
        .bind(&user.username)
        .execute(db.as_ref())
        .await?;

    Ok(UserResponse {
        username: user.username.clone(),
        email: user.email.clone(),
    })
}

pub async fn delete_user(db: Arc<PgPool>, username: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE username = $1")
        .bind(&username)
        .execute(db.as_ref())
        .await?;

    Ok(())
}
