//! Sale Models
//!
//! Represents sales transactions and their line items.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Sale entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Sale {
    pub id: String,  // UUID as TEXT

    #[sqlx(try_from = "String")]
    pub total_amount: Decimal,

    #[sqlx(try_from = "String")]
    pub paid_amount: Decimal,

    #[sqlx(try_from = "String")]
    pub change_amount: Decimal,

    pub is_loan: bool,  // Stored as INTEGER (0/1) in DB

    #[sqlx(try_from = "String")]
    pub sold_at: DateTime<Utc>,
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
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Operation {
    pub id: String,  // UUID as TEXT
    pub sale_id: String,
    pub product_id: String,
    pub product_name: String,  // Denormalized for receipts
    pub quantity: f64,

    #[sqlx(try_from = "String")]
    pub unit_price: Decimal,

    #[sqlx(try_from = "String")]
    pub subtotal: Decimal,

    #[sqlx(try_from = "String")]
    pub recorded_at: DateTime<Utc>,
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
        self.items
            .iter()
            .map(|item| Operation {
                id: uuid::Uuid::new_v4().to_string(),
                sale_id: sale_id.to_string(),
                product_id: item.product_id.clone(),
                product_name: item.product_name.clone(),
                quantity: item.quantity,
                unit_price: item.unit_price,
                subtotal: item.subtotal(),
                recorded_at: Utc::now(),
            })
            .collect()
    }
}
