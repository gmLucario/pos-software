//! Mock data structures for UI development
//!
//! These structs are used during Phase 1 to populate the UI
//! without needing a database. They will be replaced with real
//! models in later phases.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MockProduct {
    pub id: String,
    pub name: String,
    pub barcode: Option<String>,
    pub price: Decimal,
    pub cost: Option<Decimal>,
    pub stock: f64,
    pub min_stock: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MockSaleItem {
    pub product_id: String,
    pub product_name: String,
    pub quantity: f64,
    pub unit_price: Decimal,
    pub subtotal: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MockSale {
    pub id: String,
    pub items: Vec<MockSaleItem>,
    pub total: Decimal,
    pub paid: Decimal,
    pub change: Decimal,
    pub is_loan: bool,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MockLoan {
    pub id: String,
    pub sale_id: String,
    pub debtor_name: String,
    pub debtor_phone: String,
    pub total_debt: Decimal,
    pub paid_amount: Decimal,
    pub remaining: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MockLoanPayment {
    pub id: String,
    pub loan_id: String,
    pub amount: Decimal,
    pub date: DateTime<Utc>,
    pub notes: Option<String>,
}

impl MockProduct {
    /// Check if product is low on stock
    pub fn is_low_stock(&self) -> bool {
        self.stock <= self.min_stock
    }

    /// Calculate profit margin
    pub fn profit_margin(&self) -> Option<Decimal> {
        self.cost.map(|cost| {
            if cost > Decimal::ZERO {
                ((self.price - cost) / cost) * Decimal::from(100)
            } else {
                Decimal::ZERO
            }
        })
    }
}

impl MockSale {
    /// Check if sale is fully paid
    pub fn is_fully_paid(&self) -> bool {
        self.paid >= self.total
    }
}

impl MockLoan {
    /// Check if loan is fully paid
    pub fn is_paid_off(&self) -> bool {
        self.remaining <= Decimal::ZERO
    }

    /// Calculate payment percentage
    pub fn payment_percentage(&self) -> f64 {
        if self.total_debt > Decimal::ZERO {
            let percentage = (self.paid_amount / self.total_debt) * Decimal::from(100);
            percentage.to_string().parse().unwrap_or(0.0)
        } else {
            0.0
        }
    }
}

/// Generate sample mock data for development
pub fn generate_sample_data() -> (Vec<MockProduct>, Vec<MockSale>, Vec<MockLoan>) {
    use rust_decimal_macros::dec;

    // Sample products
    let products = vec![
        MockProduct {
            id: "p1".to_string(),
            name: "Coca Cola 2L".to_string(),
            barcode: Some("7501234567890".to_string()),
            price: dec!(25.50),
            cost: Some(dec!(18.00)),
            stock: 50.0,
            min_stock: 10.0,
            unit: "bottle".to_string(),
        },
        MockProduct {
            id: "p2".to_string(),
            name: "Apples".to_string(),
            barcode: None,
            price: dec!(35.00),
            cost: Some(dec!(25.00)),
            stock: 5.5,
            min_stock: 10.0,
            unit: "kg".to_string(),
        },
        MockProduct {
            id: "p3".to_string(),
            name: "Milk 1L".to_string(),
            barcode: Some("7501234567891".to_string()),
            price: dec!(22.00),
            cost: Some(dec!(16.00)),
            stock: 30.0,
            min_stock: 15.0,
            unit: "lt".to_string(),
        },
        MockProduct {
            id: "p4".to_string(),
            name: "Bread".to_string(),
            barcode: Some("7501234567892".to_string()),
            price: dec!(15.00),
            cost: Some(dec!(10.00)),
            stock: 25.0,
            min_stock: 20.0,
            unit: "unit".to_string(),
        },
    ];

    // Sample sales
    let sales = vec![
        MockSale {
            id: "s1".to_string(),
            items: vec![
                MockSaleItem {
                    product_id: "p1".to_string(),
                    product_name: "Coca Cola 2L".to_string(),
                    quantity: 2.0,
                    unit_price: dec!(25.50),
                    subtotal: dec!(51.00),
                },
                MockSaleItem {
                    product_id: "p4".to_string(),
                    product_name: "Bread".to_string(),
                    quantity: 3.0,
                    unit_price: dec!(15.00),
                    subtotal: dec!(45.00),
                },
            ],
            total: dec!(96.00),
            paid: dec!(100.00),
            change: dec!(4.00),
            is_loan: false,
            date: Utc::now(),
        },
    ];

    // Sample loans
    let loans = vec![
        MockLoan {
            id: "l1".to_string(),
            sale_id: "s2".to_string(),
            debtor_name: "Juan Pérez".to_string(),
            debtor_phone: "5551234567".to_string(),
            total_debt: dec!(250.00),
            paid_amount: dec!(100.00),
            remaining: dec!(150.00),
            status: "Partially Paid".to_string(),
            created_at: Utc::now(),
        },
        MockLoan {
            id: "l2".to_string(),
            sale_id: "s3".to_string(),
            debtor_name: "María González".to_string(),
            debtor_phone: "5559876543".to_string(),
            total_debt: dec!(180.00),
            paid_amount: dec!(50.00),
            remaining: dec!(130.00),
            status: "Active".to_string(),
            created_at: Utc::now(),
        },
    ];

    (products, sales, loans)
}
