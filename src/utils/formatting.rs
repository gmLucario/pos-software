//! Formatting Utilities
//!
//! Functions for formatting data for display in the UI.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// Format a Decimal as currency with dollar sign
///
/// # Examples
/// ```
/// use rust_decimal_macros::dec;
/// use pos_app::utils::formatting::format_currency;
///
/// assert_eq!(format_currency(dec!(10.50)), "$10.50");
/// assert_eq!(format_currency(dec!(1000.00)), "$1,000.00");
/// ```
pub fn format_currency(amount: Decimal) -> String {
    let rounded = amount.round_dp(2);
    let abs_value = rounded.abs();
    let sign = if rounded < Decimal::ZERO { "-" } else { "" };

    // Convert to string and split on decimal point
    let s = abs_value.to_string();
    let parts: Vec<&str> = s.split('.').collect();
    let integer_part = parts[0];
    let decimal_part = if parts.len() > 1 { parts[1] } else { "00" };

    // Add thousands separators
    let formatted_integer = add_thousands_separator(integer_part);

    // Ensure exactly 2 decimal places
    let padded_decimal = format!("{:0<2}", decimal_part);

    format!("{}${}.{}", sign, formatted_integer, &padded_decimal[..2])
}

/// Add thousands separator to a number string
fn add_thousands_separator(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    for (i, ch) in chars.iter().enumerate() {
        result.push(*ch);
        let pos_from_end = len - i - 1;
        if pos_from_end > 0 && pos_from_end.is_multiple_of(3) {
            result.push(',');
        }
    }

    result
}

/// Format a DateTime as a readable string
///
/// # Examples
/// ```
/// use chrono::Utc;
/// use pos_app::utils::formatting::format_datetime;
///
/// let now = Utc::now();
/// let formatted = format_datetime(&now);
/// assert!(formatted.contains(" at "));
/// ```
pub fn format_datetime(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d at %H:%M:%S").to_string()
}

/// Format a DateTime as a short date
///
/// # Examples
/// ```
/// use chrono::Utc;
/// use pos_app::utils::formatting::format_date;
///
/// let now = Utc::now();
/// let formatted = format_date(&now);
/// assert_eq!(formatted.len(), 10); // YYYY-MM-DD
/// ```
pub fn format_date(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d").to_string()
}

/// Format a DateTime as time only
///
/// # Examples
/// ```
/// use chrono::Utc;
/// use pos_app::utils::formatting::format_time;
///
/// let now = Utc::now();
/// let formatted = format_time(&now);
/// assert_eq!(formatted.len(), 8); // HH:MM:SS
/// ```
pub fn format_time(dt: &DateTime<Utc>) -> String {
    dt.format("%H:%M:%S").to_string()
}

/// Format a percentage with 1 decimal place
///
/// # Examples
/// ```
/// use pos_app::utils::formatting::format_percentage;
///
/// assert_eq!(format_percentage(0.5), "50.0%");
/// assert_eq!(format_percentage(0.333), "33.3%");
/// assert_eq!(format_percentage(1.0), "100.0%");
/// ```
pub fn format_percentage(value: f64) -> String {
    format!("{:.1}%", value * 100.0)
}

/// Format stock amount with unit
///
/// # Examples
/// ```
/// use pos_app::utils::formatting::format_stock;
///
/// assert_eq!(format_stock(10.5, "kg"), "10.50 kg");
/// assert_eq!(format_stock(5.0, "units"), "5.00 units");
/// ```
pub fn format_stock(amount: f64, unit: &str) -> String {
    format!("{:.2} {}", amount, unit)
}

/// Truncate string to max length with ellipsis
///
/// # Examples
/// ```
/// use pos_app::utils::formatting::truncate;
///
/// assert_eq!(truncate("Hello World", 5), "Hello...");
/// assert_eq!(truncate("Hi", 10), "Hi");
/// ```
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}

/// Format phone number for display
///
/// # Examples
/// ```
/// use pos_app::utils::formatting::format_phone;
///
/// assert_eq!(format_phone("1234567890"), "(123) 456-7890");
/// assert_eq!(format_phone("123456"), "123456"); // Too short
/// ```
pub fn format_phone(phone: &str) -> String {
    // Remove all non-digit characters
    let digits: String = phone.chars().filter(|c| c.is_numeric()).collect();

    // Format based on length
    match digits.len() {
        10 => format!("({}) {}-{}", &digits[0..3], &digits[3..6], &digits[6..10]),
        11 if digits.starts_with('1') => format!(
            "+{} ({}) {}-{}",
            &digits[0..1],
            &digits[1..4],
            &digits[4..7],
            &digits[7..11]
        ),
        _ => phone.to_string(), // Return as-is if format not recognized
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_format_currency() {
        assert_eq!(format_currency(dec!(10.50)), "$10.50");
        assert_eq!(format_currency(dec!(1000.00)), "$1,000.00");
        assert_eq!(format_currency(dec!(1234567.89)), "$1,234,567.89");
        assert_eq!(format_currency(dec!(0.00)), "$0.00");
        assert_eq!(format_currency(dec!(-10.50)), "-$10.50");
    }

    #[test]
    fn test_add_thousands_separator() {
        assert_eq!(add_thousands_separator("1000"), "1,000");
        assert_eq!(add_thousands_separator("1000000"), "1,000,000");
        assert_eq!(add_thousands_separator("100"), "100");
    }

    #[test]
    fn test_format_percentage() {
        assert_eq!(format_percentage(0.5), "50.0%");
        assert_eq!(format_percentage(0.333), "33.3%");
        assert_eq!(format_percentage(1.0), "100.0%");
    }

    #[test]
    fn test_format_stock() {
        assert_eq!(format_stock(10.5, "kg"), "10.50 kg");
        assert_eq!(format_stock(5.0, "units"), "5.00 units");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("Hello World", 5), "Hello...");
        assert_eq!(truncate("Hi", 10), "Hi");
        assert_eq!(truncate("Exact", 5), "Exact");
    }

    #[test]
    fn test_format_phone() {
        assert_eq!(format_phone("1234567890"), "(123) 456-7890");
        assert_eq!(format_phone("11234567890"), "+1 (123) 456-7890");
        assert_eq!(format_phone("123-456-7890"), "(123) 456-7890");
    }
}
