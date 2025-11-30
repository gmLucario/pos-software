//! Tests for mock data without requiring GUI dependencies
//!
//! These tests can run in any environment, including CI/CD.

use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

// Duplicate the mock data structs here to avoid importing dioxus-desktop
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct MockProduct {
    pub id: String,
    pub name: String,
    pub barcode: Option<String>,
    pub price: Decimal,
    pub cost: Option<Decimal>,
    pub stock: f64,
    pub min_stock: f64,
    pub unit: String,
}

impl MockProduct {
    pub fn is_low_stock(&self) -> bool {
        self.stock <= self.min_stock
    }

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

fn create_sample_products() -> Vec<MockProduct> {
    vec![
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
    ]
}

#[test]
fn test_product_creation() {
    let products = create_sample_products();
    assert_eq!(products.len(), 4, "Should have 4 products");
}

#[test]
fn test_product_low_stock_detection() {
    let products = create_sample_products();

    // Find apples (should be low on stock: 5.5 <= 10.0)
    let apples = products.iter().find(|p| p.name == "Apples").unwrap();
    assert!(apples.is_low_stock(), "Apples should be low on stock");

    // Find cola (should NOT be low on stock: 50.0 > 10.0)
    let cola = products.iter().find(|p| p.name == "Coca Cola 2L").unwrap();
    assert!(!cola.is_low_stock(), "Cola should not be low on stock");
}

#[test]
fn test_product_profit_margin_calculation() {
    let products = create_sample_products();

    let cola = products.iter().find(|p| p.name == "Coca Cola 2L").unwrap();
    let margin = cola.profit_margin().expect("Should have cost price");

    // Margin = (25.50 - 18.00) / 18.00 * 100 = 41.6666...%
    // Round to 2 decimal places for comparison
    let margin_rounded = margin.round_dp(2);
    assert_eq!(margin_rounded, dec!(41.67), "Profit margin calculation incorrect");
}

#[test]
fn test_product_with_barcode() {
    let products = create_sample_products();

    let cola = products.iter().find(|p| p.name == "Coca Cola 2L").unwrap();
    assert!(cola.barcode.is_some(), "Cola should have a barcode");
    assert_eq!(cola.barcode.as_ref().unwrap(), "7501234567890");

    let apples = products.iter().find(|p| p.name == "Apples").unwrap();
    assert!(apples.barcode.is_none(), "Apples should not have a barcode");
}

#[test]
fn test_decimal_price_precision() {
    let product = MockProduct {
        id: "test".to_string(),
        name: "Test Product".to_string(),
        barcode: None,
        price: dec!(10.99),
        cost: Some(dec!(7.50)),
        stock: 100.0,
        min_stock: 10.0,
        unit: "unit".to_string(),
    };

    // Test that Decimal maintains precision
    assert_eq!(product.price.to_string(), "10.99");
    assert_eq!(product.cost.unwrap().to_string(), "7.50");

    // Test arithmetic with Decimals
    let profit = product.price - product.cost.unwrap();
    assert_eq!(profit, dec!(3.49));
}

#[test]
fn test_serialization() {
    let product = MockProduct {
        id: "p1".to_string(),
        name: "Test".to_string(),
        barcode: Some("123".to_string()),
        price: dec!(10.50),
        cost: Some(dec!(7.00)),
        stock: 50.0,
        min_stock: 10.0,
        unit: "unit".to_string(),
    };

    // Serialize to JSON
    let json = serde_json::to_string(&product).unwrap();
    assert!(json.contains("Test"));
    assert!(json.contains("10.50"));

    // Deserialize back
    let deserialized: MockProduct = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, product);
}

#[test]
fn test_profit_margin_with_zero_cost() {
    let product = MockProduct {
        id: "test".to_string(),
        name: "Free Item".to_string(),
        barcode: None,
        price: dec!(10.00),
        cost: Some(Decimal::ZERO),
        stock: 100.0,
        min_stock: 10.0,
        unit: "unit".to_string(),
    };

    let margin = product.profit_margin().unwrap();
    assert_eq!(margin, Decimal::ZERO, "Margin should be zero when cost is zero");
}

#[test]
fn test_product_without_cost() {
    let product = MockProduct {
        id: "test".to_string(),
        name: "No Cost Item".to_string(),
        barcode: None,
        price: dec!(10.00),
        cost: None,
        stock: 100.0,
        min_stock: 10.0,
        unit: "unit".to_string(),
    };

    assert!(product.profit_margin().is_none(), "Should return None when no cost");
}
