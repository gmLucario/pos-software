//! Sales Handler
//!
//! UI event handlers for sales processing.

use crate::api::{SaleWithOperations, SalesApi, SalesStats};
use crate::models::{Sale, SaleInput};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct SalesHandler {
    api: Arc<SalesApi>,
}

impl SalesHandler {
    pub fn new(api: Arc<SalesApi>) -> Self {
        Self { api }
    }

    /// Process a new sale
    pub async fn process_sale(&self, input: SaleInput) -> Result<Sale, String> {
        self.api.process_sale(input).await
    }

    /// Get sale details with operations
    pub async fn get_sale_details(&self, id: String) -> Result<SaleWithOperations, String> {
        self.api.get_sale(&id).await
    }

    /// Load all sales
    pub async fn load_sales(&self) -> Result<Vec<Sale>, String> {
        self.api.list_sales().await
    }

    /// Get today's sales
    pub async fn get_today_sales(&self) -> Result<Vec<Sale>, String> {
        self.api.get_today_sales().await
    }

    /// Get sales within a date range
    pub async fn get_sales_by_date_range(
        &self,
        start: String,
        end: String,
    ) -> Result<Vec<Sale>, String> {
        self.api.get_sales_by_date(&start, &end).await
    }

    /// Search sales by customer name
    pub async fn search_customer_sales(&self, customer_name: String) -> Result<Vec<Sale>, String> {
        self.api.get_customer_sales(&customer_name).await
    }

    /// Get overall sales statistics
    pub async fn get_sales_stats(&self) -> Result<SalesStats, String> {
        self.api.get_sales_stats().await
    }

    /// Get today's sales statistics
    pub async fn get_today_stats(&self) -> Result<SalesStats, String> {
        self.api.get_today_stats().await
    }
}
