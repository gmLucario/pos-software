//! Product Model
//!
//! Represents inventory items with pricing and stock information.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::str::FromStr;

/// Product entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: String,  // UUID as TEXT
    pub barcode: Option<String>,
    pub full_name: String,

    #[sqlx(try_from = "String")]
    pub user_price: Decimal,  // Stored as TEXT in DB

    #[sqlx(try_from = "Option<String>")]
    pub cost_price: Option<Decimal>,  // Stored as TEXT in DB

    pub min_amount: f64,  // Minimum stock threshold
    pub current_amount: f64,  // Current inventory
    pub unit_measurement_id: i32,

    #[sqlx(try_from = "String")]
    pub created_at: DateTime<Utc>,

    #[sqlx(try_from = "String")]
    pub updated_at: DateTime<Utc>,
}

impl Product {
    /// Check if product is low on stock
    pub fn is_low_stock(&self) -> bool {
        self.current_amount <= self.min_amount
    }

    /// Calculate profit margin percentage
    pub fn profit_margin(&self) -> Option<Decimal> {
        self.cost_price.map(|cost| {
            if cost > Decimal::ZERO {
                ((self.user_price - cost) / cost) * Decimal::from(100)
            } else {
                Decimal::ZERO
            }
        })
    }

    /// Calculate profit amount
    pub fn profit_amount(&self) -> Option<Decimal> {
        self.cost_price.map(|cost| self.user_price - cost)
    }
}

/// Product creation/update data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductInput {
    pub barcode: Option<String>,
    pub full_name: String,
    pub user_price: Decimal,
    pub cost_price: Option<Decimal>,
    pub min_amount: f64,
    pub current_amount: f64,
    pub unit_measurement_id: i32,
}

impl ProductInput {
    /// Create a new Product from this input
    pub fn to_product(&self) -> Product {
        Product {
            id: uuid::Uuid::new_v4().to_string(),
            barcode: self.barcode.clone(),
            full_name: self.full_name.clone(),
            user_price: self.user_price,
            cost_price: self.cost_price,
            min_amount: self.min_amount,
            current_amount: self.current_amount,
            unit_measurement_id: self.unit_measurement_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
