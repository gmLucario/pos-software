//! Repository Traits
//!
//! Defines the interfaces for data access operations.

use crate::models::*;
use async_trait::async_trait;

/// Pagination result wrapper
#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total_count: i64,
    pub page: i64,
    pub page_size: i64,
}

/// Product repository trait
#[async_trait]
pub trait ProductRepository: Send + Sync {
    /// Create a new product
    async fn create(&self, input: ProductInput) -> Result<Product, String>;

    /// Get product by ID
    async fn get_by_id(&self, id: &str) -> Result<Option<Product>, String>;

    /// Get product by barcode
    async fn get_by_barcode(&self, barcode: &str) -> Result<Option<Product>, String>;

    /// List all products
    async fn list_all(&self) -> Result<Vec<Product>, String>;

    /// List products with pagination
    async fn list_paginated(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<PaginatedResult<Product>, String>;

    /// Update product
    async fn update(&self, id: &str, input: ProductInput) -> Result<Product, String>;

    /// Delete product
    async fn delete(&self, id: &str) -> Result<(), String>;

    /// Update stock amount
    async fn update_stock(&self, id: &str, new_amount: f64) -> Result<(), String>;

    /// Search products by name or barcode
    async fn search(&self, query: &str) -> Result<Vec<Product>, String>;

    /// Search products with pagination
    async fn search_paginated(
        &self,
        query: &str,
        page: i64,
        page_size: i64,
    ) -> Result<PaginatedResult<Product>, String>;

    /// Get low stock products
    async fn get_low_stock(&self) -> Result<Vec<Product>, String>;
}

/// Sale repository trait
#[async_trait]
pub trait SaleRepository: Send + Sync {
    /// Create a new sale with operations
    async fn create(&self, input: SaleInput) -> Result<Sale, String>;

    /// Get sale by ID
    async fn get_by_id(&self, id: &str) -> Result<Option<Sale>, String>;

    /// List all sales
    async fn list_all(&self) -> Result<Vec<Sale>, String>;

    /// Get operations for a sale
    async fn get_operations(&self, sale_id: &str) -> Result<Vec<Operation>, String>;

    /// List sales within date range
    async fn list_by_date_range(&self, start: &str, end: &str) -> Result<Vec<Sale>, String>;

    /// Get sales for a specific customer (by debtor name)
    async fn get_by_customer(&self, customer_name: &str) -> Result<Vec<Sale>, String>;
}

/// Loan repository trait
#[async_trait]
pub trait LoanRepository: Send + Sync {
    /// Create a new loan
    async fn create(
        &self,
        input: LoanInput,
        total_debt: rust_decimal::Decimal,
        paid_amount: rust_decimal::Decimal,
    ) -> Result<Loan, String>;

    /// Get loan by ID
    async fn get_by_id(&self, id: &str) -> Result<Option<Loan>, String>;

    /// List all loans
    async fn list_all(&self) -> Result<Vec<Loan>, String>;

    /// Update loan status
    async fn update_status(&self, id: &str, status_id: i32) -> Result<(), String>;

    /// Record a payment
    async fn record_payment(&self, input: LoanPaymentInput) -> Result<LoanPayment, String>;

    /// Get payments for a loan
    async fn get_payments(&self, loan_id: &str) -> Result<Vec<LoanPayment>, String>;

    /// Get active loans
    async fn get_active(&self) -> Result<Vec<Loan>, String>;

    /// Get loans by status
    async fn get_by_status(&self, status_id: i32) -> Result<Vec<Loan>, String>;

    /// Search loans by debtor name or phone
    async fn search(&self, query: &str) -> Result<Vec<Loan>, String>;
}

/// Catalog repository trait
#[async_trait]
pub trait CatalogRepository: Send + Sync {
    /// Get all unit measurements
    async fn get_units(&self) -> Result<Vec<UnitMeasurement>, String>;

    /// Get all item conditions
    async fn get_conditions(&self) -> Result<Vec<ItemCondition>, String>;

    /// Get all loan statuses
    async fn get_loan_statuses(&self) -> Result<Vec<StatusLoan>, String>;
}
