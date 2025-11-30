//! SQLite Sale Repository Implementation

use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::models::{Sale, Operation, SaleInput};
use crate::repo::SaleRepository;

pub struct SqliteSaleRepository {
    pool: SqlitePool,
}

impl SqliteSaleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SaleRepository for SqliteSaleRepository {
    async fn create(&self, input: SaleInput) -> Result<Sale, String> {
        let sale = input.to_sale();
        let operations = input.to_operations(&sale.id);

        // Start transaction
        let mut tx = self.pool.begin()
            .await
            .map_err(|e| format!("Failed to start transaction: {}", e))?;

        // Insert sale
        sqlx::query(
            r#"
            INSERT INTO sale (
                id, item_condition_id, total_amount, paid_amount,
                change_amount, created_at
            )
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&sale.id)
        .bind(sale.item_condition_id)
        .bind(sale.total_amount.to_string())
        .bind(sale.paid_amount.to_string())
        .bind(sale.change_amount.to_string())
        .bind(sale.created_at.to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Failed to insert sale: {}", e))?;

        // Insert operations
        for operation in &operations {
            sqlx::query(
                r#"
                INSERT INTO operation (
                    id, sale_id, product_id, quantity, unit_price,
                    subtotal, created_at
                )
                VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&operation.id)
            .bind(&operation.sale_id)
            .bind(&operation.product_id)
            .bind(operation.quantity)
            .bind(operation.unit_price.to_string())
            .bind(operation.subtotal.to_string())
            .bind(operation.created_at.to_rfc3339())
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Failed to insert operation: {}", e))?;

            // Update product stock
            sqlx::query(
                r#"
                UPDATE product
                SET current_amount = current_amount - ?,
                    updated_at = ?
                WHERE id = ?
                "#,
            )
            .bind(operation.quantity)
            .bind(operation.created_at.to_rfc3339())
            .bind(&operation.product_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Failed to update product stock: {}", e))?;
        }

        // Commit transaction
        tx.commit()
            .await
            .map_err(|e| format!("Failed to commit transaction: {}", e))?;

        Ok(sale)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Sale>, String> {
        let sale = sqlx::query_as::<_, Sale>(
            "SELECT * FROM sale WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to get sale by id: {}", e))?;

        Ok(sale)
    }

    async fn list_all(&self) -> Result<Vec<Sale>, String> {
        let sales = sqlx::query_as::<_, Sale>(
            "SELECT * FROM sale ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to list sales: {}", e))?;

        Ok(sales)
    }

    async fn get_operations(&self, sale_id: &str) -> Result<Vec<Operation>, String> {
        let operations = sqlx::query_as::<_, Operation>(
            "SELECT * FROM operation WHERE sale_id = ? ORDER BY created_at"
        )
        .bind(sale_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to get operations: {}", e))?;

        Ok(operations)
    }

    async fn list_by_date_range(&self, start: &str, end: &str) -> Result<Vec<Sale>, String> {
        let sales = sqlx::query_as::<_, Sale>(
            r#"
            SELECT * FROM sale
            WHERE created_at BETWEEN ? AND ?
            ORDER BY created_at DESC
            "#
        )
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to list sales by date range: {}", e))?;

        Ok(sales)
    }

    async fn get_by_customer(&self, customer_name: &str) -> Result<Vec<Sale>, String> {
        let search_term = format!("%{}%", customer_name);

        let sales = sqlx::query_as::<_, Sale>(
            r#"
            SELECT s.* FROM sale s
            JOIN loan l ON s.id = l.id
            WHERE l.debtor_name LIKE ?
            ORDER BY s.created_at DESC
            "#
        )
        .bind(&search_term)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to get sales by customer: {}", e))?;

        Ok(sales)
    }
}
