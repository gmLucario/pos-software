//! [`iced::Element`]s to be used in the stats view

use iced::{
    alignment::Horizontal,
    widget::{button, column, row, text, Text},
    Alignment, Element, Length,
};
use iced_aw::{helpers::date_picker, Card};

use crate::{
    constants::{COLUMN_PADDING, SIZE_TEXT, SIZE_TEXT_LABEL, SPACE_COLUMNS, SPACE_ROWS},
    events::AppEvent,
    kinds::AppDatePicker,
    schemas::sale_info::{SaleInfoSearchSchema, SaleInfoStats, SaleInfoWidgetsStates},
};

/// General view items
pub fn view<'a>(
    search_info: &'a SaleInfoSearchSchema,
    widgets_states: &'a SaleInfoWidgetsStates,
    data_stats: &'a SaleInfoStats,
) -> Element<'a, AppEvent> {
    let get_total_label = |msg: String| -> Text {
        text(msg)
            .size(SIZE_TEXT)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center)
    };

    column!(
        row!(
            date_picker(
                widgets_states.show_start_date,
                search_info.start_date,
                button(text(search_info.start_date).size(SIZE_TEXT_LABEL)).on_press(
                    AppEvent::ShowDatePicker(true, AppDatePicker::SaleStartDatePicker)
                ),
                AppEvent::ShowDatePicker(false, AppDatePicker::SaleStartDatePicker),
                |date| AppEvent::SubmitDatePicker(date, AppDatePicker::SaleStartDatePicker),
            ),
            date_picker(
                widgets_states.show_end_date,
                search_info.end_date,
                button(text(search_info.end_date).size(SIZE_TEXT_LABEL)).on_press(
                    AppEvent::ShowDatePicker(true, AppDatePicker::SaleEndDatePicker,)
                ),
                AppEvent::ShowDatePicker(false, AppDatePicker::SaleEndDatePicker),
                |date| AppEvent::SubmitDatePicker(date, AppDatePicker::SaleEndDatePicker),
            ),
        )
        .spacing(SPACE_ROWS),
        row!(
            Card::new(
                text("Ganancias Totales"),
                get_total_label(format!("${}", data_stats.earnings))
            )
            .foot(text(
                "ganancias considerando ventas, prestamos y productos en almacen"
            )),
            Card::new(
                text("Ventas"),
                get_total_label(format!(
                    "#{number}: ${money}",
                    number = data_stats.sales,
                    money = data_stats.total_sales
                ))
            )
            .foot(text("suma de todas las ventas y la ganancia de estas"))
        )
        .spacing(SPACE_ROWS),
        Card::new(
            text("Prestamos"),
            get_total_label(format!(
                "#{number}: ${money}",
                number = data_stats.loans,
                money = data_stats.total_loans
            ))
        )
        .foot(text("suma de todos los préstamos y pérdidas de estas"))
    )
    .padding(COLUMN_PADDING)
    .spacing(SPACE_COLUMNS)
    .align_items(Alignment::Center)
    .into()
}
