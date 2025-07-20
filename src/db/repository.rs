use tokio_postgres::{Client, Error};
use super::models::User;

pub async fn init_db(client: &Client) -> Result<(), Error> {
    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL UNIQUE
        )",
        &[],
    ).await?;
    Ok(())
}

pub async fn get_users(client: &Client) -> Result<Vec<User>, Error> {
    let rows = client.query("SELECT id, name, email FROM users", &[]).await?;

    let users = rows.iter().map(|row| User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
    }).collect();

    Ok(users)
}

pub async fn create_user(client: &Client, user: &User) -> Result<(), Error> {
    client.execute(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[&user.name, &user.email],
    ).await?;

    Ok(())
}