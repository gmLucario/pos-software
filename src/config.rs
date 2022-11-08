//! Represent parameterizable values ​​of the application

use async_std::sync::Mutex;
use once_cell::sync::Lazy;
use serde::Deserialize;

/// Parameterizable values ​​of the application
#[derive(Deserialize, Debug)]
pub struct Config {
    /// Name of the database
    pub db_name: String,
    /// User name of the database
    pub db_user: String,
    /// User name's password of the database
    pub db_password: String,
    /// Port used to connection for the database
    pub db_port: String,
    /// Number of connections allowed in the database
    pub max_connections: String,
}

impl Config {
    /// Get the database connection url
    pub fn get_db_url(&self) -> String {
        let db_url = format!(
            "postgres://{db_user}:{db_password}@localhost:{db_port}/{db_name}",
            db_user = self.db_user,
            db_password = self.db_password,
            db_port = self.db_port,
            db_name = self.db_name
        );

        db_url
    }
}

/// Singleton instance of [`crate::config::Config`]
pub static APP_CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    let config = envy::from_env::<Config>().unwrap();
    Mutex::new(config)
});
