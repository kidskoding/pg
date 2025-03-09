use std::fmt::Display;
use sha2::{Digest, Sha256};

pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub hashed_password: String,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {{ username: {}, email: {}, password: {}, hashed_password: {} }}",
            self.username, self.email, self.password, self.hashed_password
        )
    }
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(password.as_bytes());
        let hashed_password = hex::encode(hasher.finalize());

        Self {
            username,
            email,
            password,
            hashed_password,
        }
    }

    pub fn get_contents(&self) -> [&str; 4] {
        [
            &self.username,
            &self.email,
            &self.password,
            &self.hashed_password
        ]
    }
}
