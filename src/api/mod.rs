//! API Module
//!
//! Core business logic layer that orchestrates repository operations.

pub mod inventory_api;
pub mod sales_api;
pub mod loans_api;

pub use inventory_api::{InventoryApi, InventoryStats};
pub use sales_api::{SalesApi, SaleWithOperations, SalesStats};
pub use loans_api::{LoansApi, LoanWithPayments, LoanStats};
