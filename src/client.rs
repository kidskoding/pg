use std::io;
use reqwest::Client;
use serde_json::Value;
use crate::users::{user::User, user_response::UserResponse};

async fn make_get_request(endpoint: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(format!("http://{}", endpoint))
        .await?;

    let status = response.status();
    if !status.is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }

    let body = response.text().await?;
    let json_value: Value = serde_json::from_str(&body)?;

    if json_value.is_array() {
        let users: Vec<UserResponse> = serde_json::from_str(&body)?;
        Ok(format!("{} - {:?}", status, users))
    } else if json_value.is_object() {
        let user: UserResponse = serde_json::from_str(&body)?;
        Ok(format!("{} - {:?}", status, user))
    } else {
        Err("Invalid response format".into())
    }
}

async fn make_post_request(endpoint: &str, data: &User) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("http://{}", endpoint);

    let response = client.post(url)
        .json(&data)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if status != reqwest::StatusCode::CREATED {
        return Err(format!("{} - Failed to create user: {}", status, body).into());
    }

    Ok(format!("{} - User created successfully", status))
}

pub async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("Input your HTTP requests!");
    println!("An HTTP request should be in the format: <HTTP_METHOD> <ENDPOINT>");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        input = input.trim().parse::<String>()?;
        if input == "exit" || input == "quit" || input == "q" {
            break;
        }

        let vec: Vec<&str> = input.split_whitespace().collect();
        if vec.len() != 2 {
            println!("Invalid HTTP request please try again.");
            println!("An HTTP request should be in the format: <HTTP_METHOD> <ENDPOINT>");
            continue;
        }

        let (http_req, endpoint) = (vec[0], vec[1]);

        match http_req {
            "GET" => {
                let response = make_get_request(endpoint)
                    .await?;
                println!("GET {}: {}", endpoint, response);
            }
            "POST" => {
                println!("Please input your data");
                println!("Data should be in the format: username email password");
                let mut info = String::new();

                io::stdin()
                    .read_line(&mut info)
                    .expect("failed to read line");

                info = info.trim().parse::<String>()?;

                let vec: Vec<&str> = info.split_whitespace().collect();
                if vec.len() != 3 {
                    println!("Invalid input please try again. Please input a username, email, and password");
                    continue;
                }

                let user = User::new(
                    vec[0].to_string(),
                    vec[1].to_string(),
                    vec[2].to_string()
                );

                let response = make_post_request(endpoint, &user)
                    .await?;
                println!("POST {}: {}", endpoint, response);
            }
            _ => {
                eprintln!("Invalid HTTP request");
                continue;
            }
        }

        input.clear();
    }

    Ok(())
}
