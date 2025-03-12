use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {{ username: {}, email: {}, password: {} }}",
            self.username, self.email, self.password
        )
    }
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        User {
            username,
            email,
            password,
        }
    }
}
