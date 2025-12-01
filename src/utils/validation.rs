//! Validation Utilities
//!
//! Common validation functions for user input.

use rust_decimal::Decimal;

/// Validate product name
pub fn validate_product_name(name: &str) -> Result<(), String> {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return Err("Product name cannot be empty".to_string());
    }

    if trimmed.len() < 2 {
        return Err("Product name must be at least 2 characters".to_string());
    }

    if trimmed.len() > 200 {
        return Err("Product name is too long (max 200 characters)".to_string());
    }

    Ok(())
}

/// Validate barcode format
pub fn validate_barcode(barcode: &str) -> Result<(), String> {
    let trimmed = barcode.trim();

    if trimmed.is_empty() {
        return Ok(()); // Barcode is optional
    }

    if trimmed.len() < 3 {
        return Err("Barcode must be at least 3 characters".to_string());
    }

    if trimmed.len() > 50 {
        return Err("Barcode is too long (max 50 characters)".to_string());
    }

    // Check if alphanumeric with allowed special chars (-, _)
    if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err("Barcode can only contain letters, numbers, hyphens, and underscores".to_string());
    }

    Ok(())
}

/// Validate price (must be positive)
pub fn validate_price(price: Decimal) -> Result<(), String> {
    if price <= Decimal::ZERO {
        return Err("Price must be greater than zero".to_string());
    }

    if price > Decimal::new(999999999, 2) {
        return Err("Price is too large".to_string());
    }

    Ok(())
}

/// Validate stock amount (cannot be negative)
pub fn validate_stock_amount(amount: f64) -> Result<(), String> {
    if amount < 0.0 {
        return Err("Stock amount cannot be negative".to_string());
    }

    if amount > 999999.0 {
        return Err("Stock amount is too large".to_string());
    }

    Ok(())
}

/// Validate quantity for sale (must be positive)
pub fn validate_quantity(quantity: f64) -> Result<(), String> {
    if quantity <= 0.0 {
        return Err("Quantity must be greater than zero".to_string());
    }

    if quantity > 999999.0 {
        return Err("Quantity is too large".to_string());
    }

    Ok(())
}

/// Validate debtor name
pub fn validate_debtor_name(name: &str) -> Result<(), String> {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return Err("Debtor name cannot be empty".to_string());
    }

    if trimmed.len() < 2 {
        return Err("Debtor name must be at least 2 characters".to_string());
    }

    if trimmed.len() > 200 {
        return Err("Debtor name is too long (max 200 characters)".to_string());
    }

    Ok(())
}

/// Validate phone number
pub fn validate_phone(phone: &str) -> Result<(), String> {
    let trimmed = phone.trim();

    if trimmed.is_empty() {
        return Ok(()); // Phone is optional
    }

    if trimmed.len() < 7 {
        return Err("Phone number must be at least 7 digits".to_string());
    }

    if trimmed.len() > 20 {
        return Err("Phone number is too long (max 20 characters)".to_string());
    }

    // Check if contains mostly digits (allowing spaces, hyphens, parentheses, plus)
    let digit_count = trimmed.chars().filter(|c| c.is_numeric()).count();
    if digit_count < 7 {
        return Err("Phone number must contain at least 7 digits".to_string());
    }

    Ok(())
}

/// Validate payment amount (must be non-negative)
pub fn validate_payment_amount(amount: Decimal) -> Result<(), String> {
    if amount < Decimal::ZERO {
        return Err("Payment amount cannot be negative".to_string());
    }

    if amount > Decimal::new(999999999, 2) {
        return Err("Payment amount is too large".to_string());
    }

    Ok(())
}

/// Parse decimal from string with validation
pub fn parse_decimal(input: &str) -> Result<Decimal, String> {
    input.trim()
        .parse::<Decimal>()
        .map_err(|_| format!("Invalid number: {}", input))
}

/// Parse float from string with validation
pub fn parse_float(input: &str) -> Result<f64, String> {
    input.trim()
        .parse::<f64>()
        .map_err(|_| format!("Invalid number: {}", input))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_validate_product_name() {
        assert!(validate_product_name("Rice").is_ok());
        assert!(validate_product_name("  Rice  ").is_ok());
        assert!(validate_product_name("").is_err());
        assert!(validate_product_name("R").is_err());
    }

    #[test]
    fn test_validate_barcode() {
        assert!(validate_barcode("").is_ok()); // Optional
        assert!(validate_barcode("ABC123").is_ok());
        assert!(validate_barcode("AB").is_err()); // Too short
        assert!(validate_barcode("ABC@123").is_err()); // Invalid char
    }

    #[test]
    fn test_validate_price() {
        assert!(validate_price(dec!(10.50)).is_ok());
        assert!(validate_price(dec!(0.01)).is_ok());
        assert!(validate_price(dec!(0.00)).is_err());
        assert!(validate_price(dec!(-5.00)).is_err());
    }

    #[test]
    fn test_validate_phone() {
        assert!(validate_phone("").is_ok()); // Optional
        assert!(validate_phone("123-456-7890").is_ok());
        assert!(validate_phone("(555) 123-4567").is_ok());
        assert!(validate_phone("123").is_err()); // Too few digits
    }
}
