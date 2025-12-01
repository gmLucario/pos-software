//! API Module
//!
//! Core business logic layer that orchestrates repository operations.

pub mod inventory_api;
pub mod loans_api;
pub mod sales_api;

pub use inventory_api::{InventoryApi, InventoryStats};
pub use loans_api::{LoanStats, LoanWithPayments, LoansApi};
pub use sales_api::{SaleWithOperations, SalesApi, SalesStats};
