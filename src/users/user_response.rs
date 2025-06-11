use std::fmt::Display;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserResponse {
    pub username: String,
    pub email: String,
}

impl Display for UserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {{ username: {}, email: {} }}",
            self.username, self.email
        )
    }
}
