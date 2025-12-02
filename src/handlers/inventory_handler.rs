//! Inventory Handler
//!
//! UI event handlers for inventory management.

use crate::api::{InventoryApi, InventoryStats};
use crate::models::{Product, ProductInput, UnitMeasurement};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct InventoryHandler {
    api: Arc<InventoryApi>,
}

impl InventoryHandler {
    pub fn new(api: Arc<InventoryApi>) -> Self {
        Self { api }
    }

    /// Load all products
    pub async fn load_products(&self) -> Result<Vec<Product>, String> {
        self.api.list_products().await
    }

    /// Create a new product
    pub async fn create_product(&self, input: ProductInput) -> Result<Product, String> {
        self.api.create_product(input).await
    }

    /// Update an existing product
    pub async fn update_product(&self, id: String, input: ProductInput) -> Result<Product, String> {
        self.api.update_product(&id, input).await
    }

    /// Delete a product
    pub async fn delete_product(&self, id: String) -> Result<(), String> {
        self.api.delete_product(&id).await
    }

    /// Search products by name or barcode
    pub async fn search_products(&self, query: String) -> Result<Vec<Product>, String> {
        self.api.search_products(&query).await
    }

    /// Get low stock products
    pub async fn get_low_stock(&self) -> Result<Vec<Product>, String> {
        self.api.get_low_stock_products().await
    }

    /// Add stock to a product
    pub async fn add_stock(&self, id: String, quantity: f64) -> Result<Product, String> {
        self.api.add_stock(&id, quantity).await
    }

    /// Remove stock from a product
    pub async fn remove_stock(&self, id: String, quantity: f64) -> Result<Product, String> {
        self.api.remove_stock(&id, quantity).await
    }

    /// Set stock amount directly
    pub async fn set_stock(&self, id: String, amount: f64) -> Result<Product, String> {
        self.api.set_stock(&id, amount).await
    }

    /// Get inventory statistics
    pub async fn get_stats(&self) -> Result<InventoryStats, String> {
        self.api.get_inventory_stats().await
    }

    /// Get product by barcode (for quick lookup)
    pub async fn scan_barcode(&self, barcode: String) -> Result<Product, String> {
        self.api.get_product_by_barcode(&barcode).await
    }

    /// Get all unit measurements
    pub async fn get_units(&self) -> Result<Vec<UnitMeasurement>, String> {
        self.api.get_units().await
    }
}
