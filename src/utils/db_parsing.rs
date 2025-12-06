//! Database Parsing Utilities
//!
//! Helper functions for parsing database values (Decimal and DateTime from TEXT fields).

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::Row;
use std::str::FromStr;

/// Parse a Decimal from a database TEXT column
///
/// # Arguments
/// * `row` - The database row
/// * `column` - The column name
///
/// # Returns
/// * `Ok(Decimal)` on success
/// * `Err(sqlx::Error)` with appropriate column decode error
pub fn parse_decimal_from_row<'r>(
    row: &'r sqlx::sqlite::SqliteRow,
    column: &str,
) -> Result<Decimal, sqlx::Error> {
    let s: String = row.try_get(column)?;
    Decimal::from_str(&s).map_err(|e| sqlx::Error::ColumnDecode {
        index: column.to_string(),
        source: Box::new(e),
    })
}

/// Parse an optional Decimal from a database TEXT column
///
/// # Arguments
/// * `row` - The database row
/// * `column` - The column name
///
/// # Returns
/// * `Ok(Some(Decimal))` if value exists and parses successfully
/// * `Ok(None)` if value is NULL
/// * `Err(sqlx::Error)` on parse failure
pub fn parse_optional_decimal_from_row<'r>(
    row: &'r sqlx::sqlite::SqliteRow,
    column: &str,
) -> Result<Option<Decimal>, sqlx::Error> {
    let s: Option<String> = row.try_get(column)?;
    s.map(|s| Decimal::from_str(&s))
        .transpose()
        .map_err(|e| sqlx::Error::ColumnDecode {
            index: column.to_string(),
            source: Box::new(e),
        })
}

/// Parse a DateTime<Utc> from a database TEXT column (RFC3339 format)
///
/// # Arguments
/// * `row` - The database row
/// * `column` - The column name
///
/// # Returns
/// * `Ok(DateTime<Utc>)` on success
/// * `Err(sqlx::Error)` with appropriate column decode error
pub fn parse_datetime_from_row<'r>(
    row: &'r sqlx::sqlite::SqliteRow,
    column: &str,
) -> Result<DateTime<Utc>, sqlx::Error> {
    let s: String = row.try_get(column)?;
    DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| sqlx::Error::ColumnDecode {
            index: column.to_string(),
            source: Box::new(e),
        })
}

/// Calculate pagination offset
///
/// # Arguments
/// * `page` - The page number (1-indexed)
/// * `page_size` - Number of items per page
///
/// # Returns
/// The offset value for SQL OFFSET clause
#[inline]
pub fn calculate_offset(page: i64, page_size: i64) -> i64 {
    (page - 1) * page_size
}

/// Format a search query with SQL LIKE wildcards
///
/// # Arguments
/// * `query` - The search term
///
/// # Returns
/// The query wrapped in % wildcards for LIKE matching
#[inline]
pub fn format_like_pattern(query: &str) -> String {
    format!("%{}%", query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_offset() {
        assert_eq!(calculate_offset(1, 10), 0);
        assert_eq!(calculate_offset(2, 10), 10);
        assert_eq!(calculate_offset(5, 20), 80);
    }

    #[test]
    fn test_format_like_pattern() {
        assert_eq!(format_like_pattern("test"), "%test%");
        assert_eq!(format_like_pattern(""), "%%");
        assert_eq!(format_like_pattern("a b"), "%a b%");
    }
}
