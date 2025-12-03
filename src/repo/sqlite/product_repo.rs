//! SQLite Product Repository Implementation

use crate::models::{Product, ProductInput};
use crate::repo::{PaginatedResult, ProductRepository};
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct SqliteProductRepository {
    pool: SqlitePool,
}

impl SqliteProductRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepository for SqliteProductRepository {
    async fn create(&self, input: ProductInput) -> Result<Product, String> {
        let product = input.to_product();

        sqlx::query(
            r#"
            INSERT INTO product (
                id, barcode, full_name, user_price, cost_price,
                min_amount, current_amount, unit_measurement_id,
                created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&product.id)
        .bind(&product.barcode)
        .bind(&product.full_name)
        .bind(product.user_price.to_string())
        .bind(product.cost_price.map(|d| d.to_string()))
        .bind(product.min_amount)
        .bind(product.current_amount)
        .bind(product.unit_measurement_id)
        .bind(product.created_at.to_rfc3339())
        .bind(product.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to create product: {}", e))?;

        Ok(product)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Product>, String> {
        let product = sqlx::query_as::<_, Product>("SELECT * FROM product WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Failed to get product by id: {}", e))?;

        Ok(product)
    }

    async fn get_by_barcode(&self, barcode: &str) -> Result<Option<Product>, String> {
        let product = sqlx::query_as::<_, Product>("SELECT * FROM product WHERE barcode = ?")
            .bind(barcode)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Failed to get product by barcode: {}", e))?;

        Ok(product)
    }

    async fn list_all(&self) -> Result<Vec<Product>, String> {
        let products = sqlx::query_as::<_, Product>("SELECT * FROM product ORDER BY full_name")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to list products: {}", e))?;

        Ok(products)
    }

    async fn list_paginated(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<PaginatedResult<Product>, String> {
        // Get total count
        let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM product")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Failed to count products: {}", e))?;

        // Calculate offset
        let offset = (page - 1) * page_size;

        // Get paginated products
        let products = sqlx::query_as::<_, Product>(
            "SELECT * FROM product ORDER BY full_name LIMIT ? OFFSET ?",
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to list products: {}", e))?;

        Ok(PaginatedResult {
            items: products,
            total_count,
            page,
            page_size,
        })
    }

    async fn update(&self, id: &str, input: ProductInput) -> Result<Product, String> {
        let updated_at = chrono::Utc::now();

        sqlx::query(
            r#"
            UPDATE product
            SET barcode = ?, full_name = ?, user_price = ?, cost_price = ?,
                min_amount = ?, current_amount = ?, unit_measurement_id = ?,
                updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&input.barcode)
        .bind(&input.full_name)
        .bind(input.user_price.to_string())
        .bind(input.cost_price.map(|d| d.to_string()))
        .bind(input.min_amount)
        .bind(input.current_amount)
        .bind(input.unit_measurement_id)
        .bind(updated_at.to_rfc3339())
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to update product: {}", e))?;

        // Fetch and return updated product
        self.get_by_id(id)
            .await?
            .ok_or_else(|| format!("Product not found after update: {}", id))
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM product WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete product: {}", e))?;

        Ok(())
    }

    async fn update_stock(&self, id: &str, new_amount: f64) -> Result<(), String> {
        let updated_at = chrono::Utc::now();

        sqlx::query("UPDATE product SET current_amount = ?, updated_at = ? WHERE id = ?")
            .bind(new_amount)
            .bind(updated_at.to_rfc3339())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to update stock: {}", e))?;

        Ok(())
    }

    async fn search(&self, query: &str) -> Result<Vec<Product>, String> {
        let search_term = format!("%{}%", query);

        let products = sqlx::query_as::<_, Product>(
            r#"
            SELECT * FROM product
            WHERE full_name LIKE ? OR barcode LIKE ?
            ORDER BY full_name
            "#,
        )
        .bind(&search_term)
        .bind(&search_term)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to search products: {}", e))?;

        Ok(products)
    }

    async fn search_paginated(
        &self,
        query: &str,
        page: i64,
        page_size: i64,
    ) -> Result<PaginatedResult<Product>, String> {
        let search_term = format!("%{}%", query);

        // Get total count of matching products
        let total_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM product
            WHERE full_name LIKE ? OR barcode LIKE ?
            "#,
        )
        .bind(&search_term)
        .bind(&search_term)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Failed to count search results: {}", e))?;

        // Calculate offset
        let offset = (page - 1) * page_size;

        // Get paginated search results
        let products = sqlx::query_as::<_, Product>(
            r#"
            SELECT * FROM product
            WHERE full_name LIKE ? OR barcode LIKE ?
            ORDER BY full_name
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(&search_term)
        .bind(&search_term)
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to search products: {}", e))?;

        Ok(PaginatedResult {
            items: products,
            total_count,
            page,
            page_size,
        })
    }

    async fn get_low_stock(&self) -> Result<Vec<Product>, String> {
        let products = sqlx::query_as::<_, Product>(
            "SELECT * FROM product WHERE current_amount <= min_amount ORDER BY full_name",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to get low stock products: {}", e))?;

        Ok(products)
    }
}
