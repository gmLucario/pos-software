//! Setup the external domain app components

use std::fs::File;

use crate::{config, db, domain::AppProcessor, errors::AppError, result::AppResult};
use iced::Application;
use iced::Settings;
use log::LevelFilter;
use simplelog::ConfigBuilder;
use simplelog::WriteLogger;
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

/// Try to setup the logger
pub fn logger_wrapper() -> AppResult<()> {
    let logger_config = ConfigBuilder::new()
        .add_filter_allow_str("pos_app")
        .set_time_format_rfc3339()
        .build();

    let log_filename = "pos-app.log";
    let logger_file = File::options()
        .append(true)
        .create(true)
        .open(log_filename)
        .map_err(|err| {
            get_setup_error(
                "logger_wrapper",
                &format!("can't create {}", log_filename),
                &err.to_string(),
            )
        })?;

    WriteLogger::init(LevelFilter::Info, logger_config, logger_file)
        .map_err(|err| {
            get_setup_error("logger_wrapper", "can't setup the logger", &err.to_string())
        })
        .map(|_| ())
}

/// Setup the main iced app processor
pub async fn setup_main_app() -> AppResult<()> {
    AppProcessor::run(Settings::default()).map_err(|err| {
        get_setup_error(
            "setup_main_app",
            "no se pudo configurar iced::App",
            &err.to_string(),
        )
    })
}
