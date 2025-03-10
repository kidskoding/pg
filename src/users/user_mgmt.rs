use std::sync::Arc;
use tokio_postgres::Client;
use super::user::User;

pub async fn add_user(db: Arc<Client>, user: &User) -> Result<(), tokio_postgres::Error> {
    db.execute(
        "INSERT INTO users (username, email, password, password_hash) VALUES ($1, $2, $3, $4)",
        &[&user.username, &user.email, &user.password, &user.hashed_password],
    ).await?;

    Ok(())
}

pub async fn get_user(db: Arc<Client>, username: &str) -> Result<Option<User>, tokio_postgres::Error> {
    for row in db.query(
        "SELECT * FROM users",
        &[],
    ).await? {
        if row.get::<_, &str>(1) == username {
            let (email, password, password_hash)
                = (row.get::<_, &str>(2), row.get::<_, &str>(3), row.get::<_, &str>(4));

            println!("found person: {}", username);
            println!("email: {}, password (hashed): {}", email, password_hash);

            let user = Some(User {
                username: username.to_string(),
                email: email.to_string(),
                password: password.to_string(),
                hashed_password: password_hash.to_string(),
            });

            return Ok(user);
        }
    }

    Ok(None)
}

pub async fn get_users(db: Arc<Client>) -> Result<Vec<User>, tokio_postgres::Error> {
    let mut users = Vec::new();

    for row in db.query(
        "SELECT * FROM users",
        &[],
    ).await? {
        let (username, email, password, password_hash) = (
            row.get::<_, &str>(1),
            row.get::<_, &str>(2),
            row.get::<_, &str>(3),
            row.get::<_, &str>(4)
        );

        let user = User {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            hashed_password: password_hash.to_string(),
        };

        users.push(user);
    }

    Ok(users)
}
