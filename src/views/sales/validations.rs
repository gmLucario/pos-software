//! Sales Validations
//!
//! Validation functions for the sales module.

/// Validates if a payment amount string is a valid money format.
///
/// Valid formats:
/// - Empty string (for loans)
/// - Positive numbers with max 2 decimal places
/// - Examples: "100", "50.5", "25.99", "0.5", "."
///
/// Invalid formats:
/// - Negative numbers: "-10"
/// - More than 2 decimals: "10.999"
/// - Non-numeric: "abc"
/// - Multiple decimal points: "10.5.5"
pub fn is_valid_payment_amount(value: &str) -> bool {
    // Allow empty string
    if value.is_empty() {
        return true;
    }

    // Check for invalid characters (only digits and one decimal point allowed)
    let has_invalid_chars = value.chars().any(|c| !c.is_ascii_digit() && c != '.');
    if has_invalid_chars {
        return false;
    }

    // Check for multiple decimal points
    let decimal_count = value.chars().filter(|&c| c == '.').count();
    if decimal_count > 1 {
        return false;
    }

    // If it's just a decimal point, allow it (user is typing)
    if value == "." {
        return true;
    }

    // Check decimal places (max 2)
    if let Some(decimal_pos) = value.find('.') {
        let decimal_part = &value[decimal_pos + 1..];
        if decimal_part.len() > 2 {
            return false;
        }
    }

    // Must parse as a valid positive number (if not just a decimal point)
    if value != "." {
        if let Ok(num) = value.parse::<f64>() {
            num >= 0.0
        } else {
            false
        }
    } else {
        true
    }
}

/// Validates if a quantity string is valid.
///
/// Valid formats:
/// - Positive numbers with max 3 decimal places
/// - Examples: "1", "0.5", "2.250", "10.125"
///
/// Invalid formats:
/// - Zero or negative: "0", "-1"
/// - More than 3 decimals: "1.2345"
/// - Non-numeric: "abc"
pub fn is_valid_quantity(value: &str) -> bool {
    // Empty string not allowed for quantity
    if value.is_empty() {
        return false;
    }

    // Check for invalid characters (only digits and one decimal point allowed)
    let has_invalid_chars = value.chars().any(|c| !c.is_ascii_digit() && c != '.');
    if has_invalid_chars {
        return false;
    }

    // Check for multiple decimal points
    let decimal_count = value.chars().filter(|&c| c == '.').count();
    if decimal_count > 1 {
        return false;
    }

    // Just a decimal point is not valid for quantity
    if value == "." {
        return false;
    }

    // Check decimal places (max 3)
    if let Some(decimal_pos) = value.find('.') {
        let decimal_part = &value[decimal_pos + 1..];
        if decimal_part.len() > 3 {
            return false;
        }
    }

    // Must parse as a valid positive number greater than zero
    if let Ok(num) = value.parse::<f64>() {
        num > 0.0
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_payment_amounts() {
        assert!(is_valid_payment_amount("")); // Empty (loan)
        assert!(is_valid_payment_amount("0"));
        assert!(is_valid_payment_amount("100"));
        assert!(is_valid_payment_amount("50.5"));
        assert!(is_valid_payment_amount("25.99"));
        assert!(is_valid_payment_amount("0.01"));
        assert!(is_valid_payment_amount(".")); // User typing decimal
        assert!(is_valid_payment_amount("0."));
        assert!(is_valid_payment_amount(".5"));
    }

    #[test]
    fn test_invalid_payment_amounts() {
        assert!(!is_valid_payment_amount("-10")); // Negative
        assert!(!is_valid_payment_amount("-")); // Minus sign
        assert!(!is_valid_payment_amount("10.999")); // More than 2 decimals
        assert!(!is_valid_payment_amount("abc")); // Non-numeric
        assert!(!is_valid_payment_amount("10a")); // Contains letter
        assert!(!is_valid_payment_amount("a10")); // Starts with letter
        assert!(!is_valid_payment_amount("10.1.1")); // Multiple decimals
        assert!(!is_valid_payment_amount("10..5")); // Multiple decimals
        assert!(!is_valid_payment_amount("10 5")); // Space
        assert!(!is_valid_payment_amount("10,5")); // Comma
    }

    #[test]
    fn test_valid_quantities() {
        assert!(is_valid_quantity("1"));
        assert!(is_valid_quantity("0.5"));
        assert!(is_valid_quantity("2.250"));
        assert!(is_valid_quantity("10.125"));
        assert!(is_valid_quantity("100"));
        assert!(is_valid_quantity("0.001"));
    }

    #[test]
    fn test_invalid_quantities() {
        assert!(!is_valid_quantity("")); // Empty
        assert!(!is_valid_quantity("0")); // Zero
        assert!(!is_valid_quantity("-1")); // Negative
        assert!(!is_valid_quantity("1.2345")); // More than 3 decimals
        assert!(!is_valid_quantity("abc")); // Non-numeric
        assert!(!is_valid_quantity(".")); // Just decimal
        assert!(!is_valid_quantity("-0.5")); // Negative
    }
}
