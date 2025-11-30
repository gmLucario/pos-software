//! POS Application Library
//!
//! This library provides the core functionality for the POS system.
//! It's separated from main.rs to allow for testing and potential
//! library usage.

#![allow(non_snake_case)]

pub mod mock_data;
pub mod models;
pub mod utils;

#[cfg(feature = "desktop")]
pub mod views;
