//! API Module
//!
//! Core business logic layer that orchestrates repository operations.

pub mod inventory_api;
pub mod sales_api;
pub mod loans_api;

pub use inventory_api::InventoryApi;
pub use sales_api::SalesApi;
pub use loans_api::LoansApi;
