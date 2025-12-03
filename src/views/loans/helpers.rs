//! Helper Functions
//!
//! Common utilities for loan management.

/// Calculate total pages based on total count and page size
pub fn calculate_total_pages(total_count: i64, page_size: i64) -> i64 {
    ((total_count as f64) / (page_size as f64)).ceil() as i64
}
