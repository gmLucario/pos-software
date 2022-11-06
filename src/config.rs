//! Represent parameterizable values ​​of the application

use async_std::sync::Mutex;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_port: String,
    pub max_connections: String,
}

impl Config {
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

pub static APP_CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    let config = envy::from_env::<Config>().unwrap();
    Mutex::new(config)
});
