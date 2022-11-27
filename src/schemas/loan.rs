//! Data structures to get user inputs related to loan info view

use iced_aw::date_picker::Date;

#[derive(Clone)]
/// Represents user input values to search loans
pub struct LoanSearchSchema {
    /// Date start for lookig loans
    pub start_date: Date,
    /// Until this date to lookig loans
    pub end_date: Date,
    /// Client name like, to looking loans
    pub client: String,
}

impl Default for LoanSearchSchema {
    fn default() -> Self {
        let today = Date::today();
        Self {
            start_date: today,
            end_date: today,
            client: String::default(),
        }
    }
}

/// Represents states of the date picker
#[derive(Default)]
pub struct LoanWidgetsStates {
    /// If start_date picker is shown or not
    pub show_start_date: bool,
    /// If end_date picker is shown or not
    pub show_end_date: bool,
    /// If the modal of payments of a loan should be render or not
    pub show_modal: bool,
}
