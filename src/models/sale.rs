//! Sale Models
//!
//! Represents sales transactions and their line items.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Sale entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sale {
    pub id: String, // UUID as TEXT

    pub total_amount: Decimal,

    pub paid_amount: Decimal,

    pub change_amount: Decimal,

    pub is_loan: bool, // Stored as INTEGER (0/1) in DB

    pub sold_at: DateTime<Utc>,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for Sale {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use crate::utils::db_parsing::{parse_datetime_from_row, parse_decimal_from_row};
        use sqlx::Row;

        Ok(Sale {
            id: row.try_get("id")?,
            total_amount: parse_decimal_from_row(row, "total_amount")?,
            paid_amount: parse_decimal_from_row(row, "paid_amount")?,
            change_amount: parse_decimal_from_row(row, "change_amount")?,
            is_loan: row.try_get("is_loan")?,
            sold_at: parse_datetime_from_row(row, "sold_at")?,
        })
    }
}

impl Sale {
    /// Check if sale is fully paid
    pub fn is_fully_paid(&self) -> bool {
        self.paid_amount >= self.total_amount
    }

    /// Get remaining amount to be paid
    pub fn remaining_amount(&self) -> Decimal {
        if self.total_amount > self.paid_amount {
            self.total_amount - self.paid_amount
        } else {
            Decimal::ZERO
        }
    }
}

/// Operation (sale line item) entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Operation {
    pub id: String, // UUID as TEXT
    pub sale_id: String,
    pub product_id: String,
    pub product_name: String, // Denormalized for receipts
    pub quantity: f64,

    pub unit_price: Decimal,

    pub subtotal: Decimal,

    pub recorded_at: DateTime<Utc>,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for Operation {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use crate::utils::db_parsing::{parse_datetime_from_row, parse_decimal_from_row};
        use sqlx::Row;

        Ok(Operation {
            id: row.try_get("id")?,
            sale_id: row.try_get("sale_id")?,
            product_id: row.try_get("product_id")?,
            product_name: row.try_get("product_name")?,
            quantity: row.try_get("quantity")?,
            unit_price: parse_decimal_from_row(row, "unit_price")?,
            subtotal: parse_decimal_from_row(row, "subtotal")?,
            recorded_at: parse_datetime_from_row(row, "recorded_at")?,
        })
    }
}

/// Input for creating a new sale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleInput {
    pub items: Vec<SaleItemInput>,
    pub paid_amount: Decimal,
}

/// Input for a sale line item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleItemInput {
    pub product_id: String,
    pub product_name: String,
    pub quantity: f64,
    pub unit_price: Decimal,
}

impl SaleItemInput {
    pub fn subtotal(&self) -> Decimal {
        self.unit_price * Decimal::from_f64_retain(self.quantity).unwrap_or_default()
    }
}

impl SaleInput {
    /// Calculate total amount
    pub fn total_amount(&self) -> Decimal {
        self.items.iter().map(|item| item.subtotal()).sum()
    }

    /// Calculate change
    pub fn change_amount(&self) -> Decimal {
        let total = self.total_amount();
        if self.paid_amount > total {
            self.paid_amount - total
        } else {
            Decimal::ZERO
        }
    }

    /// Check if this sale should be a loan
    pub fn is_loan(&self) -> bool {
        self.paid_amount < self.total_amount()
    }

    /// Convert to Sale entity
    pub fn to_sale(&self) -> Sale {
        Sale {
            id: uuid::Uuid::new_v4().to_string(),
            total_amount: self.total_amount(),
            paid_amount: self.paid_amount,
            change_amount: self.change_amount(),
            is_loan: self.is_loan(),
            sold_at: Utc::now(),
        }
    }

    /// Convert items to Operation entities
    pub fn to_operations(&self, sale_id: &str) -> Vec<Operation> {
        let now = Utc::now();
        self.items
            .iter()
            .map(|item| Operation {
                id: uuid::Uuid::new_v4().to_string(),
                sale_id: sale_id.to_owned(),
                product_id: item.product_id.to_owned(),
                product_name: item.product_name.to_owned(),
                quantity: item.quantity,
                unit_price: item.unit_price,
                subtotal: item.subtotal(),
                recorded_at: now,
            })
            .collect()
    }
}
