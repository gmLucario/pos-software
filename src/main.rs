//! Entry point of the application

pub mod config;
pub mod constants;
pub mod controllers;
pub mod db;
pub mod domain;
pub mod errors;
pub mod events;
pub mod helpers;
pub mod kinds;
pub mod models;
pub mod repo;
pub mod result;
pub mod schemas;
pub mod setup;
pub mod views;

#[async_std::main]
async fn main() -> result::GenericReturn<()> {
    setup::setup_app_config()?;

    setup::setup_db(config::AppConfig::get()).await?;

    setup::setup_main_app().await?;

    Ok(())
}
