//! Handle logic to link [`crate::views::sales_info`]
//! module with the [`crate::data::sale_repo::SaleRepo`]

use iced_aw::date_picker::Date;

use crate::{
    kinds::SaleInfoDatePicker,
    schemas::sale_info::{SaleInfoSearchSchema, SaleInfoStats, SaleInfoWidgetsStates},
};

#[derive(Default)]
/// Data to be render in [`crate::views::sales_info`] view
pub struct SaleInfoData {
    /// Data to be used to search the loans
    pub search_info: SaleInfoSearchSchema,
    /// states to show the date pickers
    pub widgets_states: SaleInfoWidgetsStates,
    /// Data to be shown as stats
    pub data_stats: SaleInfoStats,
}

#[derive(Default)]
/// SaleInfo controller
pub struct SaleInfo {
    /// Data to be render in [`crate::views::sales_info`] view
    pub data: SaleInfoData,
}

impl SaleInfo {
    /// Set the state to a datepicker
    pub fn set_state_datepicker(&mut self, date_picker: SaleInfoDatePicker, state: bool) {
        match date_picker {
            SaleInfoDatePicker::StartDatePicker => self.data.widgets_states.show_start_date = state,
            SaleInfoDatePicker::EndDatePicker => self.data.widgets_states.show_end_date = state,
        }
    }

    /// Set a value to a date picker and hide it
    pub fn set_datepicker_value(&mut self, date_picker: SaleInfoDatePicker, date: Date) {
        match date_picker {
            SaleInfoDatePicker::StartDatePicker => {
                self.data.search_info.start_date = date;
                self.data.widgets_states.show_start_date = false;
            }
            SaleInfoDatePicker::EndDatePicker => {
                self.data.search_info.end_date = date;
                self.data.widgets_states.show_end_date = false;
            }
        }
    }
}
