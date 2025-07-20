mod api;
mod config;
mod db;

use std::net::TcpListener;
use std::io::{Read, Write};
use tokio_postgres::NoTls;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    // Initialize database configuration
    let db_config = config::database::DatabaseConfig::new();

    // Connect to the database
    let (client, connection) = tokio_postgres::connect(
        &db_config.connection_string(),
        NoTls
    ).await?;

    // Spawn connection handling
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Initialize database
    db::repository::init_db(&client).await?;

    // Start HTTP server
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server running on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                stream.read(&mut buffer)?;

                let request = String::from_utf8_lossy(&buffer[..]);
                let response = api::handler::handle_request(&request, &client).await;

                stream.write(response.as_bytes())?;
                stream.flush()?;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}