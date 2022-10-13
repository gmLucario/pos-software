pub mod config;
pub mod constants;
pub mod controllers;
pub mod data;
pub mod db;
pub mod domain;
pub mod kinds;
pub mod models;
pub mod queries;
pub mod schemas;
pub mod views;

use iced::{Application, Settings};
use sqlx::postgres::PgPoolOptions;

use crate::config::APP_CONFIG;
use crate::db::{Db, INSTANCE_DB};
use crate::domain::App;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&APP_CONFIG.lock().await.get_db_url())
        .await?;
    let pool_db = Db::with_pool_connection(pool);
    INSTANCE_DB.set(pool_db).unwrap();

    App::run(Settings::default())?;

    Ok(())
}
