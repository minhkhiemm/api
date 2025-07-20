use tokio_postgres::Client;
use crate::api::response::create_response;
use crate::db::{models::User, repository};

pub async fn handle_request(request: &str, client: &Client) -> String {
    let request_line = request.lines().next().unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    if parts.len() < 2 {
        return create_response(404, "Not Found");
    }

    match (parts[0], parts[1]) {
        ("GET", "/users") => {
            match repository::get_users(client).await {
                Ok(users) => {
                    let json = serde_json::to_string(&users).unwrap();
                    create_response(200, &json)
                }
                Err(_) => create_response(500, "Internal Server Error"),
            }
        }
        ("POST", "/users") => {
            if let Some(body) = request.split("\r\n\r\n").nth(1) {
                match serde_json::from_str::<User>(body) {
                    Ok(user) => {
                        match repository::create_user(client, &user).await {
                            Ok(_) => create_response(201, "User created"),
                            Err(_) => create_response(500, "Internal Server Error"),
                        }
                    }
                    Err(_) => create_response(400, "Invalid JSON"),
                }
            } else {
                create_response(400, "Missing body")
            }
        }
        _ => create_response(404, "Not Found"),
    }
}