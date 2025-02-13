use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Debug)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            max_connections: 15,
            min_connections: 5,
        }
    }
}

impl DatabaseConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    pub fn with_min_connections(mut self, min: u32) -> Self {
        self.min_connections = min;
        self
    }
}

#[allow(dead_code)]
pub fn establish_connection_pool() -> DbPool {
    establish_connection_pool_with_config(DatabaseConfig::new())
}

pub fn establish_connection_pool_with_config(config: DatabaseConfig) -> DbPool {
    let manager = ConnectionManager::<MysqlConnection>::new(config.url);
    r2d2::Pool::builder()
        .max_size(config.max_connections) // Use max_connections from config
        .min_idle(Some(config.min_connections)) // Use min_connections from config
        .connection_timeout(std::time::Duration::from_secs(10)) // Set connection timeout
        .test_on_check_out(true) // Test connections when they're retrieved
        .build(manager)
        .expect("Failed to create pool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_config_defaults() {
        std::env::set_var("DATABASE_URL", "test_url");
        let config = DatabaseConfig::default();
        assert_eq!(config.max_connections, 15);
        assert_eq!(config.min_connections, 5);
        assert_eq!(config.url, "test_url");
    }

    #[test]
    fn test_database_config_builder() {
        std::env::set_var("DATABASE_URL", "test_url");
        let config = DatabaseConfig::new()
            .with_max_connections(20)
            .with_min_connections(10);
        assert_eq!(config.max_connections, 20);
        assert_eq!(config.min_connections, 10);
    }
}
