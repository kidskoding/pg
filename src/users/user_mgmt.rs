use std::sync::Arc;
use sha2::{Sha256, Digest};
use tokio_postgres::Client;
use super::{user::User, user_response::UserResponse};

pub async fn add_user(db: Arc<Client>, user: &User) -> Result<(), tokio_postgres::Error> {
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(user.password.as_bytes());
    let hashed_password = hex::encode(hasher.finalize());

    db.execute(
        "INSERT INTO users (username, email, hashed_password) VALUES ($1, $2, $3)",
        &[&user.username, &user.email, &hashed_password],
    ).await?;

    Ok(())
}

pub async fn get_user(db: Arc<Client>, username: &str)
    -> Result<Option<UserResponse>, tokio_postgres::Error> {

    for row in db.query(
        "SELECT * FROM users",
        &[],
    ).await? {
        if row.get::<_, &str>(1) == username {
            let email = row.get::<_, &str>(2);

            let user = Some(UserResponse {
                username: username.to_string(),
                email: email.to_string(),
            });

            return Ok(user);
        }
    }

    Ok(None)
}

pub async fn get_users(db: Arc<Client>)
    -> Result<Vec<UserResponse>, tokio_postgres::Error> {

    let mut users = Vec::new();

    for row in db.query(
        "SELECT * FROM users",
        &[],
    ).await? {
        let (username, email) = (
            row.get::<_, &str>(1),
            row.get::<_, &str>(2),
        );

        let user = UserResponse {
            username: username.to_string(),
            email: email.to_string(),
        };

        users.push(user);
    }

    Ok(users)
}

pub async fn update_user(
    db: Arc<Client>,
    user: &User
) -> Result<UserResponse, tokio_postgres::Error> {
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(user.password.as_bytes());
    let hashed_password = hex::encode(hasher.finalize());

    db.execute(
        "UPDATE users SET email = $1, hashed_password = $2 WHERE username = $3",
        &[&user.email, &hashed_password, &user.username],
    ).await?;

    Ok(UserResponse {
        username: user.username.clone(),
        email: user.email.clone(),
    })
}

pub async fn delete_user(
    db: Arc<Client>,
    username: &str,
) -> Result<(), tokio_postgres::Error> {
    db.execute(
        "DELETE FROM users WHERE username = $1",
        &[&username],
    ).await?;

    Ok(())
}
