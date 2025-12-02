//! Inventory API
//!
//! Business logic for product and inventory management.

use crate::models::{Product, ProductInput, UnitMeasurement};
use crate::repo::{CatalogRepository, ProductRepository, PaginatedResult};
use std::sync::Arc;

#[derive(Clone)]
pub struct InventoryApi {
    product_repo: Arc<dyn ProductRepository>,
    catalog_repo: Arc<dyn CatalogRepository>,
}

impl std::fmt::Debug for InventoryApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InventoryApi").finish()
    }
}

impl PartialEq for InventoryApi {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.product_repo, &other.product_repo)
            && Arc::ptr_eq(&self.catalog_repo, &other.catalog_repo)
    }
}

impl InventoryApi {
    pub fn new(
        product_repo: Arc<dyn ProductRepository>,
        catalog_repo: Arc<dyn CatalogRepository>,
    ) -> Self {
        Self {
            product_repo,
            catalog_repo,
        }
    }

    /// Create a new product with validation
    pub async fn create_product(&self, input: ProductInput) -> Result<Product, String> {
        // Validate input
        if input.full_name.trim().is_empty() {
            return Err("Product name cannot be empty".to_string());
        }

        if input.user_price <= rust_decimal::Decimal::ZERO {
            return Err("Product price must be greater than zero".to_string());
        }

        if input.min_amount < 0.0 {
            return Err("Minimum amount cannot be negative".to_string());
        }

        if input.current_amount < 0.0 {
            return Err("Current amount cannot be negative".to_string());
        }

        // Check for duplicate barcode if provided
        if let Some(ref barcode) = input.barcode {
            if !barcode.is_empty() && (self.product_repo.get_by_barcode(barcode).await?).is_some() {
                return Err(format!("Product with barcode '{}' already exists", barcode));
            }
        }

        // Create product
        self.product_repo.create(input).await
    }

    /// Get product by ID
    pub async fn get_product(&self, id: &str) -> Result<Product, String> {
        self.product_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| format!("Product not found: {}", id))
    }

    /// Get product by barcode
    pub async fn get_product_by_barcode(&self, barcode: &str) -> Result<Product, String> {
        self.product_repo
            .get_by_barcode(barcode)
            .await?
            .ok_or_else(|| format!("Product not found with barcode: {}", barcode))
    }

    /// List all products
    pub async fn list_products(&self) -> Result<Vec<Product>, String> {
        self.product_repo.list_all().await
    }

    /// List products with pagination
    pub async fn list_products_paginated(&self, page: i64, page_size: i64) -> Result<PaginatedResult<Product>, String> {
        if page < 1 {
            return Err("Page number must be at least 1".to_string());
        }

        if page_size < 1 || page_size > 100 {
            return Err("Page size must be between 1 and 100".to_string());
        }

        self.product_repo.list_paginated(page, page_size).await
    }

    /// Update product with validation
    pub async fn update_product(&self, id: &str, input: ProductInput) -> Result<Product, String> {
        // Validate existence
        let _ = self.get_product(id).await?;

        // Validate input (same as create)
        if input.full_name.trim().is_empty() {
            return Err("Product name cannot be empty".to_string());
        }

        if input.user_price <= rust_decimal::Decimal::ZERO {
            return Err("Product price must be greater than zero".to_string());
        }

        // Check for duplicate barcode (excluding current product)
        if let Some(ref barcode) = input.barcode {
            if !barcode.is_empty() {
                if let Some(existing) = self.product_repo.get_by_barcode(barcode).await? {
                    if existing.id != id {
                        return Err(format!(
                            "Another product with barcode '{}' already exists",
                            barcode
                        ));
                    }
                }
            }
        }

        self.product_repo.update(id, input).await
    }

    /// Delete product
    pub async fn delete_product(&self, id: &str) -> Result<(), String> {
        // Validate existence
        let _ = self.get_product(id).await?;

        self.product_repo.delete(id).await
    }

    /// Add stock to product
    pub async fn add_stock(&self, id: &str, quantity: f64) -> Result<Product, String> {
        if quantity <= 0.0 {
            return Err("Quantity must be positive".to_string());
        }

        let product = self.get_product(id).await?;
        let new_amount = product.current_amount + quantity;

        self.product_repo.update_stock(id, new_amount).await?;
        self.get_product(id).await
    }

    /// Remove stock from product
    pub async fn remove_stock(&self, id: &str, quantity: f64) -> Result<Product, String> {
        if quantity <= 0.0 {
            return Err("Quantity must be positive".to_string());
        }

        let product = self.get_product(id).await?;
        let new_amount = product.current_amount - quantity;

        if new_amount < 0.0 {
            return Err("Insufficient stock".to_string());
        }

        self.product_repo.update_stock(id, new_amount).await?;
        self.get_product(id).await
    }

    /// Set stock amount directly
    pub async fn set_stock(&self, id: &str, amount: f64) -> Result<Product, String> {
        if amount < 0.0 {
            return Err("Stock amount cannot be negative".to_string());
        }

        let _ = self.get_product(id).await?;

        self.product_repo.update_stock(id, amount).await?;
        self.get_product(id).await
    }

    /// Search products
    pub async fn search_products(&self, query: &str) -> Result<Vec<Product>, String> {
        if query.trim().is_empty() {
            return self.list_products().await;
        }

        self.product_repo.search(query).await
    }

    /// Get low stock products
    pub async fn get_low_stock_products(&self) -> Result<Vec<Product>, String> {
        self.product_repo.get_low_stock().await
    }

    /// Get inventory statistics
    pub async fn get_inventory_stats(&self) -> Result<InventoryStats, String> {
        let products = self.product_repo.list_all().await?;
        let low_stock = self.product_repo.get_low_stock().await?;

        let total_products = products.len();
        let low_stock_count = low_stock.len();

        let total_value = products
            .iter()
            .map(|p| {
                p.user_price
                    * rust_decimal::Decimal::from_f64_retain(p.current_amount).unwrap_or_default()
            })
            .sum();

        let total_cost = products
            .iter()
            .filter_map(|p| p.cost_price)
            .zip(products.iter().map(|p| p.current_amount))
            .map(|(cost, amount)| {
                cost * rust_decimal::Decimal::from_f64_retain(amount).unwrap_or_default()
            })
            .sum();

        Ok(InventoryStats {
            total_products,
            low_stock_count,
            total_value,
            total_cost,
        })
    }
    /// Get all unit measurements
    pub async fn get_units(&self) -> Result<Vec<UnitMeasurement>, String> {
        self.catalog_repo.get_units().await
    }
}

/// Inventory statistics
#[derive(Debug, Clone)]
pub struct InventoryStats {
    pub total_products: usize,
    pub low_stock_count: usize,
    pub total_value: rust_decimal::Decimal,
    pub total_cost: rust_decimal::Decimal,
}
