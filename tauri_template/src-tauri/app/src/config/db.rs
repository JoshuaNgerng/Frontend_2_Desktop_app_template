use crate::utils::parsing::parse_bool;
use std::str::FromStr;
use sqlx::sqlite::SqliteConnectOptions;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub max_connections: u32,
    pub enable_logging: bool,
    pub opts: SqliteConnectOptions 
}

impl DbConfig {
    pub fn load() -> Self {
        let database_url = std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set");
        let opts = SqliteConnectOptions::from_str(&database_url)
            .expect("cannot get sqlite options")
            .create_if_missing(true);
        Self {
            max_connections: std::env::var("DB_MAX_CONN")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .expect("DB_MAX_CONN must be a valid number"),

            enable_logging: std::env::var("ENABLE_LOGGING")
                .ok()
                .and_then(|v| parse_bool(&v))
                .unwrap_or(false),
            opts
        }
    }
}
