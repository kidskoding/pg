use postgres::Client;
use super::user::User;

pub fn add_user(db: &mut Client, user: &User) -> Result<(), postgres::Error> {
    db.execute(
        "INSERT INTO users (username, email, password, password_hash) VALUES ($1, $2, $3, $4)",
        &[&user.username, &user.email, &user.password, &user.hashed_password],
    )?;

    Ok(())
}

pub fn get_user(db: &mut Client, username: &str) -> Result<Option<User>, postgres::Error> {
    for row in db.query(
        "SELECT * FROM users",
        &[],
    )? {
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
