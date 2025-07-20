use std::env;

pub struct DatabaseConfig {
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub name: String,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        DatabaseConfig {
            host: env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string()),
            user: env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string()),
            password: env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string()),
            name: env::var("POSTGRES_DB").unwrap_or_else(|_| "api_db".to_string()),
        }
    }

    pub fn connection_string(&self) -> String {
        format!(
            "host={} port={} user={} password={} dbname={}",
            self.host, self.port, self.user, self.password, self.name
        )
    }
}