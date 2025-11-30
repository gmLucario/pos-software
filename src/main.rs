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

        // Launch the Dioxus desktop app
        dioxus_desktop::launch(App);
    }
}
