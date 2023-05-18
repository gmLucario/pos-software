//! Setup the external domain app components

use crate::{config, db, domain::AppProcessor, errors::AppError, result::AppResult};
use iced::Application;
use iced::Settings;
use sqlx::postgres::PgPoolOptions;

/// Get a [crate::errors::AppError] of type [crate::errors::ErrorType::SetUpError] with a custom `raw_msg`
fn get_setup_error(function_name: &str, msg: &str, raw_msg: &str) -> AppError {
    AppError::db_error(&format!("src/setup.rs::{function_name}"), msg, raw_msg)
}

/// Set up the database connection
pub async fn setup_db(app_config: &config::AppConfig) -> AppResult<()> {
    let result = PgPoolOptions::new()
        .max_connections(app_config.max_connections)
        .connect(&app_config.database_url)
        .await;

    match result {
        Ok(pool) => {
            if db::INSTANCE
                .set(db::AppDb::with_pool_connection(pool))
                .is_err()
            {
                return Err(get_setup_error(
                    "setup_db",
                    "ya ha sido seteado el 'AppDb'",
                    "wrong use singleton AppDb",
                ));
            }
            Ok(())
        }
        Err(err) => Err(get_setup_error(
            "setup_db",
            "no se pudo conectar a la bd",
            &err.to_string(),
        )),
    }
}

/// Load app config values
pub fn setup_app_config() -> AppResult<()> {
    let config = config::AppConfig::from_env()?;
    if config::INSTANCE.set(config).is_err() {
        return Err(get_setup_error(
            "setup_app_config",
            "ya ha sido seteado el 'AppConfig'",
            "wrong use singleton AppConfig",
        ));
    }
    Ok(())
}

pub async fn setup_main_app() -> AppResult<()> {
    AppProcessor::run(Settings::default()).map_err(|err| {
        get_setup_error(
            "setup_main_app",
            "no se pudo configurar iced::App",
            &err.to_string(),
        )
    })
}
