//! POS Application - Point of Sale System
//!
//! A desktop application for managing inventory, sales, and loans.
//! Built with Dioxus for cross-platform support (macOS, Windows).

#![allow(non_snake_case)]

#[cfg(feature = "desktop")]
use dioxus::prelude::*;
#[cfg(feature = "desktop")]
use pos_app::views::App;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() {
    #[cfg(not(feature = "desktop"))]
    {
        eprintln!("Error: This binary requires the 'desktop' feature.");
        eprintln!("Build with: cargo build --features desktop");
        std::process::exit(1);
    }

    #[cfg(feature = "desktop")]
    {
        // Initialize logging
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set tracing subscriber");

        tracing::info!("Starting POS Application");

        // Initialize database
        let runtime = tokio::runtime::Runtime::new()
            .expect("Failed to create Tokio runtime");

        let pool = runtime.block_on(async {
            let db_url = pos_app::utils::db::get_database_url();
            tracing::info!("Initializing database at: {}", db_url);

            pos_app::utils::db::initialize_database(&db_url)
                .await
                .expect("Failed to initialize database")
        });

        tracing::info!("Database initialized successfully");

        // Create app state
        let app_state = pos_app::handlers::AppState::new(pool);
        tracing::info!("Application state created");

        // Launch the Dioxus desktop app with app state
        launch(|| rsx! {
            App { app_state: app_state.clone() }
        });
    }
}
