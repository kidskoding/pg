extern crate serde_json;
use reqwest::Client;
use crate::users::{user::User, user_response::UserResponse};

async fn make_get_request(endpoint: &str) -> Result<UserResponse, Box<dyn std::error::Error>> {
    let response = reqwest::get(endpoint)
        .await?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}",
            response.status()).into());
    }

    let body = response.json::<UserResponse>().await?;
    Ok(body)
}

async fn make_post_request(endpoint: &str, data: &User) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.post(endpoint)
        .json(&data)
        .send()
        .await?;

    if response.status() != reqwest::StatusCode::CREATED {
        return Err(format!("Failed to create user, status: {}",
            response.status()).into());
    }

    Ok("User created successfully".to_string())
}

pub async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    let user = User {
        username: "jdoe".to_string(),
        email: "jdoe@example.com".to_string(),
        password: "rust".to_string(),
    };

    println!("POST http://localhost:3000/users: {:?}", make_post_request("http://localhost:3000/users", &user)
        .await?);

    let response = make_get_request(
        format!("http://localhost:3000/users/{}", user.username)
            .as_str()
    ).await?;
    println!("GET http://localhost:3000/users/{}: {:?}", user.username, response);

    Ok(())
}
