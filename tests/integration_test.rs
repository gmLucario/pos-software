//! Integration tests for POS application
//!
//! These tests validate the mock data and business logic
//! without requiring a GUI display.

use pos_app::mock_data::{generate_sample_data, MockProduct};
use rust_decimal_macros::dec;

#[test]
fn test_mock_data_generation() {
    let (products, sales, loans) = generate_sample_data();

    // Verify products
    assert!(!products.is_empty(), "Should have sample products");
    assert_eq!(products.len(), 4, "Should have 4 sample products");

    // Check first product
    let cola = &products[0];
    assert_eq!(cola.name, "Coca Cola 2L");
    assert_eq!(cola.price, dec!(25.50));
    assert_eq!(cola.stock, 50.0);
    assert_eq!(cola.unit, "bottle");

    // Verify sales
    assert!(!sales.is_empty(), "Should have sample sales");

    // Verify loans
    assert!(!loans.is_empty(), "Should have sample loans");
    assert_eq!(loans.len(), 2, "Should have 2 sample loans");
}

#[test]
fn test_product_low_stock_detection() {
    let (products, _, _) = generate_sample_data();

    // Find apples (should be low on stock)
    let apples = products.iter().find(|p| p.name == "Apples").unwrap();
    assert!(apples.is_low_stock(), "Apples should be low on stock (5.5 <= 10.0)");

    // Find cola (should NOT be low on stock)
    let cola = products.iter().find(|p| p.name == "Coca Cola 2L").unwrap();
    assert!(!cola.is_low_stock(), "Cola should not be low on stock (50.0 > 10.0)");
}

#[test]
fn test_product_profit_margin() {
    let (products, _, _) = generate_sample_data();

    let cola = products.iter().find(|p| p.name == "Coca Cola 2L").unwrap();
    let margin = cola.profit_margin().expect("Should have cost price");

    // Margin = (25.50 - 18.00) / 18.00 * 100 = 41.67%
    let expected_margin = dec!(41.666666666666666666666666667);
    assert_eq!(margin, expected_margin, "Profit margin calculation incorrect");
}

#[test]
fn test_loan_payment_percentage() {
    let (_, _, loans) = generate_sample_data();

    let loan = &loans[0];
    assert_eq!(loan.debtor_name, "Juan Pérez");
    assert_eq!(loan.total_debt, dec!(250.00));
    assert_eq!(loan.paid_amount, dec!(100.00));
    assert_eq!(loan.remaining, dec!(150.00));

    let percentage = loan.payment_percentage();
    assert_eq!(percentage, 40.0, "Payment percentage should be 40%");
}

#[test]
fn test_loan_paid_off_status() {
    let (_, _, loans) = generate_sample_data();

    // All sample loans should not be paid off
    for loan in &loans {
        assert!(!loan.is_paid_off(), "Sample loans should not be fully paid");
    }
}

#[test]
fn test_product_with_barcode() {
    let (products, _, _) = generate_sample_data();

    let cola = products.iter().find(|p| p.name == "Coca Cola 2L").unwrap();
    assert!(cola.barcode.is_some(), "Cola should have a barcode");
    assert_eq!(cola.barcode.as_ref().unwrap(), "7501234567890");

    let apples = products.iter().find(|p| p.name == "Apples").unwrap();
    assert!(apples.barcode.is_none(), "Apples should not have a barcode");
}
