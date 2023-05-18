//! Represent parameterizable values ​​of the application

use crate::{errors::AppError, result::AppResult};
use once_cell::sync::OnceCell;
use serde::Deserialize;

/// Define the default value for `are_values_valid` in [crate::config::Config]
fn default_are_values_valid() -> bool {
    false
}

/// Parameterizable values ​​of the application
#[derive(Deserialize, Debug, Default, Clone, PartialEq)]
pub struct AppConfig {
    /// Database connection url
    pub database_url: String,
    /// Number of connections allowed in the database
    pub max_connections: u32,
    /// If env variables were set correctly
    #[serde(default = "default_are_values_valid")]
    are_values_valid: bool,
}

pub static INSTANCE: OnceCell<AppConfig> = OnceCell::new();

impl AppConfig {
    pub fn get() -> &'static AppConfig {
        INSTANCE.get().expect("enviroment variables not set")
    }

    pub fn from_env() -> AppResult<Self> {
        envy::from_env::<AppConfig>().map_err(|err| {
            AppError::setup_error(
                "src/config.rs::AppConfig::from_env",
                "Variables de ambiente no definidas",
                &err.to_string(),
            )
        })
    }
}
