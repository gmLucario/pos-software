//! Helper Functions
//!
//! Common utilities and calculations for inventory management.

use crate::models::Product;
use rust_decimal::Decimal;

/// Calculate statistics for a list of products
pub struct InventoryStats {
    pub total_count: i64,
    pub low_stock_count: usize,
    pub total_value: Decimal,
}

impl InventoryStats {
    pub fn calculate(products: &[Product], total_count: i64) -> Self {
        let low_stock_count = products.iter().filter(|p| p.is_low_stock()).count();

        let total_value: Decimal = products
            .iter()
            .map(|p| {
                p.user_price * Decimal::from_f64_retain(p.current_amount).unwrap_or_default()
            })
            .sum();

        Self {
            total_count,
            low_stock_count,
            total_value,
        }
    }
}

/// Calculate total pages based on total count and page size
pub fn calculate_total_pages(total_count: i64, page_size: i64) -> i64 {
    ((total_count as f64) / (page_size as f64)).ceil() as i64
}

/// Determine if we're in search mode based on pagination info
pub fn is_search_mode(pagination_info: &Option<(i64, i64)>) -> bool {
    pagination_info.is_none()
}

/// Get the appropriate stat labels based on mode
pub fn get_stat_labels(is_search: bool) -> (&'static str, &'static str) {
    if is_search {
        ("Matching Products", "Search Value")
    } else {
        ("Total Products", "Total Value")
    }
}

/// Get the appropriate empty state message
pub fn get_empty_message(is_search: bool) -> &'static str {
    if is_search {
        "No products found matching your search."
    } else {
        "No products found. Add some products to get started!"
    }
}
