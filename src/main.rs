//! POS Application - Point of Sale System
//!
//! A desktop application for managing inventory, sales, and loans.
//! Built with Dioxus for cross-platform support (macOS, Windows).

#![allow(non_snake_case)]

mod mock_data;
mod views;

use dioxus::prelude::*;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    tracing::info!("Starting POS Application");

    // Launch the Dioxus desktop app
    dioxus_desktop::launch(views::App);
}
