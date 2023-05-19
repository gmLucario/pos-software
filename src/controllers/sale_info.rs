//! Handle logic to link [`crate::views::sales_info`]
//! module with the [`crate::repo::sale_repo`]

use iced_aw::date_picker::Date;

use crate::schemas::sale_info::{SaleInfoSearchSchema, SaleInfoStats, SaleInfoWidgetsStates};

#[derive(Default)]
/// SaleInfo controller
pub struct SaleInfo {
    /// Data to be used to search the loans
    pub search_info: SaleInfoSearchSchema,
    /// states to show the date pickers
    pub widgets_states: SaleInfoWidgetsStates,
    /// Data to be shown as stats
    pub data_stats: SaleInfoStats,
}

impl SaleInfo {
    /// Show/Hide start date picker
    pub fn show_start_date(&mut self, to_show: bool) {
        self.widgets_states.show_start_date = to_show;
    }

    /// Show/Hide end date picker
    pub fn show_end_date(&mut self, to_show: bool) {
        self.widgets_states.show_end_date = to_show;
    }

    /// Set start date picker value
    pub fn set_start_date_value(&mut self, value: Date) {
        self.search_info.start_date = value;
    }

    /// Set end date picker value
    pub fn set_end_date_value(&mut self, value: Date) {
        self.search_info.end_date = value;
    }
}
