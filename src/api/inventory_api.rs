//! Inventory API
//!
//! Business logic for product and inventory management.

use crate::models::{Product, ProductInput};
use crate::repo::ProductRepository;
use std::sync::Arc;

pub struct InventoryApi {
    product_repo: Arc<dyn ProductRepository>,
}

impl InventoryApi {
    pub fn new(product_repo: Arc<dyn ProductRepository>) -> Self {
        Self { product_repo }
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
            if !barcode.is_empty() {
                if let Some(_) = self.product_repo.get_by_barcode(barcode).await? {
                    return Err(format!("Product with barcode '{}' already exists", barcode));
                }
            }
        }

        // Create product
        self.product_repo.create(input).await
    }

    /// Get product by ID
    pub async fn get_product(&self, id: &str) -> Result<Product, String> {
        self.product_repo.get_by_id(id).await?
            .ok_or_else(|| format!("Product not found: {}", id))
    }

    /// Get product by barcode
    pub async fn get_product_by_barcode(&self, barcode: &str) -> Result<Product, String> {
        self.product_repo.get_by_barcode(barcode).await?
            .ok_or_else(|| format!("Product not found with barcode: {}", barcode))
    }

    /// List all products
    pub async fn list_products(&self) -> Result<Vec<Product>, String> {
        self.product_repo.list_all().await
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
                        return Err(format!("Another product with barcode '{}' already exists", barcode));
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

        let total_value = products.iter()
            .map(|p| p.user_price * rust_decimal::Decimal::from_f64_retain(p.current_amount).unwrap_or_default())
            .sum();

        let total_cost = products.iter()
            .filter_map(|p| p.cost_price)
            .zip(products.iter().map(|p| p.current_amount))
            .map(|(cost, amount)| cost * rust_decimal::Decimal::from_f64_retain(amount).unwrap_or_default())
            .sum();

        Ok(InventoryStats {
            total_products,
            low_stock_count,
            total_value,
            total_cost,
        })
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
