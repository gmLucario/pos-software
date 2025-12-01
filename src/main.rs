//! POS Application - Point of Sale System
//!
//! A desktop application for managing inventory, sales, and loans.
//! Built with Dioxus for cross-platform support (macOS, Windows).

#![allow(non_snake_case)]

pub mod api;
pub mod handlers;
pub mod models;
pub mod repo;
pub mod utils;
pub mod views;

use dioxus::prelude::*;
use std::sync::OnceLock;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

static APP_STATE: OnceLock<handlers::AppState> = OnceLock::new();

fn main() {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    tracing::info!("Starting POS Application");

    // Initialize database
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    let pool = runtime.block_on(async {
        let db_url = utils::db::get_database_url();
        tracing::info!("Initializing database at: {}", db_url);

        utils::db::initialize_database(&db_url)
            .await
            .expect("Failed to initialize database")
    });

    tracing::info!("Database initialized successfully");

    // Create app state and store it globally
    APP_STATE
        .set(handlers::AppState::new(pool))
        .expect("Failed to set app state");
    tracing::info!("Application state created");

    // Launch the Dioxus desktop app
    launch(app_root);
}

fn app_root() -> Element {
    let app_state = APP_STATE.get().expect("App state not initialized").clone();

    rsx! {
        views::app::App { app_state }
    }
}
