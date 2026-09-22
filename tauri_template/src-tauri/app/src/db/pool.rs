use sqlx::{ConnectOptions, SqlitePool, sqlite::SqlitePoolOptions};
use crate::config::db::DbConfig;

pub async fn create_pool(config: &DbConfig) -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .connect_with(config.opts.clone())
        .await
        .expect(&format!("Failed to connect DB, {}", config.opts.to_url_lossy().as_str()))
}
