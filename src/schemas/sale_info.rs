//! Data structures to get user inputs related to sale stats/info
use chrono::NaiveDate;
use iced_aw::core::date::succ_year;
use iced_aw::date_picker::Date;
use sqlx::types::BigDecimal;

#[derive(Clone)]
/// Represents user input values to search sale stats
pub struct SaleInfoSearchSchema {
    /// Date start for lookig loans
    pub start_date: Date,
    /// Until this date to lookig loans
    pub end_date: Date,
}

#[derive(Default)]
/// Represents data stats about money earning/invested
pub struct SaleInfoStats {
    /// balance about earnings
    pub earnings: BigDecimal,
    /// number of sales
    pub sales: i64,
    /// total amount of money of sales
    pub total_sales: BigDecimal,
    /// number of loans
    pub loans: i64,
    /// total amount of money of loans
    pub total_loans: BigDecimal,
}

impl Default for SaleInfoSearchSchema {
    fn default() -> Self {
        let today = Date::today();
        let end_date = Date::from(succ_year(NaiveDate::from(today)));

        Self {
            start_date: Date {
                year: today.year,
                month: 1,
                day: 1,
            },
            end_date: Date {
                year: end_date.year,
                month: 1,
                day: 1,
            },
        }
    }
}

/// Represents states of the date picker
#[derive(Default)]
pub struct SaleInfoWidgetsStates {
    /// If start_date picker is shown or not
    pub show_start_date: bool,
    /// If end_date picker is shown or not
    pub show_end_date: bool,
}
