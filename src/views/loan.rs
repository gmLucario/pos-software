use iced::{
    widget::{button, column, row, text, text_input},
    Alignment, Element, Length,
};
use iced_aw::DatePicker;

use crate::{
    constants::{COLUMN_PADDING, SIZE_TEXT_INPUT, SIZE_TEXT_LABEL, SPACE_COLUMNS, SPACE_ROWS},
    kinds::{AppEvents, LoanDatePicker, LoanInputs},
    schemas::loan::{LoanDatePickerStates, LoanSchema},
    views::fonts,
};

#[derive(Default)]
pub struct LoanView {}

impl LoanView {
    pub fn search_results<'a>(
        states: &'a LoanDatePickerStates,
        info: &'a LoanSchema,
    ) -> Element<'a, AppEvents> {
        let mut search_btn = button(text('\u{f00c}').font(fonts::FONT_ICONS))
            .style(crate::style::btns::get_style_btn_ok());

        if !info.client.is_empty() {
            search_btn = search_btn.on_press(AppEvents::LoanSearchRequested);
        }

        column!(
            row!(
                DatePicker::new(
                    states.show_start_date,
                    info.start_date,
                    button(text(info.start_date).size(SIZE_TEXT_LABEL)).on_press(
                        AppEvents::LoanShowDatePicker(true, LoanDatePicker::StartDatePicker,)
                    ),
                    AppEvents::LoanShowDatePicker(false, LoanDatePicker::StartDatePicker),
                    |date| AppEvents::LoanSubmitDatePicker(date, LoanDatePicker::StartDatePicker),
                ),
                DatePicker::new(
                    states.show_end_date,
                    info.end_date,
                    button(text(info.end_date).size(SIZE_TEXT_LABEL)).on_press(
                        AppEvents::LoanShowDatePicker(true, LoanDatePicker::EndDatePicker,)
                    ),
                    AppEvents::LoanShowDatePicker(false, LoanDatePicker::EndDatePicker),
                    |date| AppEvents::LoanSubmitDatePicker(date, LoanDatePicker::EndDatePicker),
                )
            )
            .spacing(SPACE_ROWS),
            row!(
                text_input("", &info.client, |input_value| {
                    AppEvents::LoanInputChanged(input_value, LoanInputs::DebtorNameLike)
                })
                .size(SIZE_TEXT_INPUT),
                search_btn,
            )
            .spacing(SPACE_ROWS),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .padding(COLUMN_PADDING)
        .spacing(SPACE_COLUMNS)
        .align_items(Alignment::Center)
        .into()
    }
}
