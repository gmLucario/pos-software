use crate::schemas::loan::{LoanDatePickerStates, LoanSchema};

#[derive(Default)]
pub struct Loan {
    pub search_info: LoanSchema,
    pub date_picker_states: LoanDatePickerStates,
}
