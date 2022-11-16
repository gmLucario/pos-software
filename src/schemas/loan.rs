use iced_aw::date_picker::Date;

pub struct LoanSchema {
    pub start_date: Date,
    pub end_date: Date,
    pub client: String,
}

impl Default for LoanSchema {
    fn default() -> Self {
        let today = Date::today();
        Self {
            start_date: today,
            end_date: today,
            client: String::default(),
        }
    }
}

#[derive(Default)]
pub struct LoanDatePickerStates {
    pub show_start_date: bool,
    pub show_end_date: bool,
}
