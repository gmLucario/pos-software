//! Catalog Models
//!
//! Reference data models for dropdowns and lookups.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Item condition types (Good, Damaged, Expired)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct ItemCondition {
    pub id: i32,
    pub description: String,
}

/// Loan status types (Active, Partially Paid, Fully Paid, Cancelled)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct StatusLoan {
    pub id: i32,
    pub description: String,
}

/// Unit of measurement (kg, lt, unit, pcs, box, can, bottle)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct UnitMeasurement {
    pub id: i32,
    pub description: String,
    pub abbreviation: String,
}

impl ItemCondition {
    pub const GOOD: i32 = 1;
    pub const DAMAGED: i32 = 2;
    pub const EXPIRED: i32 = 3;
}

impl StatusLoan {
    pub const ACTIVE: i32 = 1;
    pub const PARTIALLY_PAID: i32 = 2;
    pub const FULLY_PAID: i32 = 3;
    pub const CANCELLED: i32 = 4;
}

impl UnitMeasurement {
    pub const KILOGRAM: i32 = 1;
    pub const LITER: i32 = 2;
    pub const UNIT: i32 = 3;
    pub const PIECE: i32 = 4;
    pub const BOX: i32 = 5;
    pub const CAN: i32 = 6;
    pub const BOTTLE: i32 = 7;
}
