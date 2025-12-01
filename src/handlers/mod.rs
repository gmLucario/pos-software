//! Handlers Module
//!
//! Bridge layer between UI (views) and business logic (API).
//! Handlers manage async operations and state updates for Dioxus components.

pub mod inventory_handler;
pub mod sales_handler;
pub mod loans_handler;

pub use inventory_handler::InventoryHandler;
pub use sales_handler::SalesHandler;
pub use loans_handler::LoansHandler;

/// Application state container
pub struct AppState {
    pub inventory_handler: InventoryHandler,
    pub sales_handler: SalesHandler,
    pub loans_handler: LoansHandler,
}

impl AppState {
    /// Create new application state with database pool
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        use crate::repo::sqlite::*;
        use crate::api::*;
        use std::sync::Arc;

        // Create repositories
        let product_repo = Arc::new(SqliteProductRepository::new(pool.clone()));
        let sale_repo = Arc::new(SqliteSaleRepository::new(pool.clone()));
        let loan_repo = Arc::new(SqliteLoanRepository::new(pool.clone()));

        // Create APIs
        let inventory_api = Arc::new(InventoryApi::new(product_repo.clone()));
        let sales_api = Arc::new(SalesApi::new(sale_repo.clone(), product_repo.clone()));
        let loans_api = Arc::new(LoansApi::new(loan_repo.clone(), sale_repo.clone()));

        // Create handlers
        Self {
            inventory_handler: InventoryHandler::new(inventory_api),
            sales_handler: SalesHandler::new(sales_api),
            loans_handler: LoansHandler::new(loans_api),
        }
    }
}
