//! SQLite Catalog Repository Implementation

use crate::models::{ItemCondition, StatusLoan, UnitMeasurement};
use crate::repo::CatalogRepository;
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct SqliteCatalogRepository {
    pool: SqlitePool,
}

impl SqliteCatalogRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CatalogRepository for SqliteCatalogRepository {
    async fn get_units(&self) -> Result<Vec<UnitMeasurement>, String> {
        let units =
            sqlx::query_as::<_, UnitMeasurement>("SELECT * FROM unit_measurement ORDER BY id")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| format!("Failed to get unit measurements: {}", e))?;

        Ok(units)
    }

    async fn get_conditions(&self) -> Result<Vec<ItemCondition>, String> {
        let conditions =
            sqlx::query_as::<_, ItemCondition>("SELECT * FROM item_condition ORDER BY id")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| format!("Failed to get item conditions: {}", e))?;

        Ok(conditions)
    }

    async fn get_loan_statuses(&self) -> Result<Vec<StatusLoan>, String> {
        let statuses = sqlx::query_as::<_, StatusLoan>("SELECT * FROM status_loan ORDER BY id")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to get loan statuses: {}", e))?;

        Ok(statuses)
    }
}
