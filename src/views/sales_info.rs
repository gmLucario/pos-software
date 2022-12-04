//! [`iced::Element`]s to be used in the stats view

use iced::{
    alignment::Horizontal,
    widget::{button, column, row, text, Text},
    Alignment, Element, Length,
};
use iced_aw::{Card, DatePicker};

use crate::{
    constants::{COLUMN_PADDING, SIZE_TEXT, SIZE_TEXT_LABEL, SPACE_COLUMNS, SPACE_ROWS},
    controllers::sale_info::SaleInfoData,
    kinds::{AppEvents, SaleInfoDatePicker},
};

/// General view items
pub fn view(data: &SaleInfoData) -> Element<'static, AppEvents> {
    let get_total_label = |msg: String| -> Text {
        text(msg)
            .size(SIZE_TEXT)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center)
    };

    column!(
        row!(
            DatePicker::new(
                data.widgets_states.show_start_date,
                data.search_info.start_date,
                button(text(data.search_info.start_date).size(SIZE_TEXT_LABEL)).on_press(
                    AppEvents::SaleInfoShowDatePicker(true, SaleInfoDatePicker::StartDatePicker,)
                ),
                AppEvents::SaleInfoShowDatePicker(false, SaleInfoDatePicker::StartDatePicker),
                |date| AppEvents::SaleInfoSubmitDatePicker(
                    date,
                    SaleInfoDatePicker::StartDatePicker
                ),
            ),
            DatePicker::new(
                data.widgets_states.show_end_date,
                data.search_info.end_date,
                button(text(data.search_info.end_date).size(SIZE_TEXT_LABEL)).on_press(
                    AppEvents::SaleInfoShowDatePicker(true, SaleInfoDatePicker::EndDatePicker,)
                ),
                AppEvents::SaleInfoShowDatePicker(false, SaleInfoDatePicker::EndDatePicker),
                |date| AppEvents::SaleInfoSubmitDatePicker(date, SaleInfoDatePicker::EndDatePicker),
            ),
        )
        .spacing(SPACE_ROWS),
        row!(
            Card::new(
                text("Ganancias Totales"),
                get_total_label(format!("${}", data.data_stats.earnings))
            )
            .foot(text(
                "ganancias considerando ventas, prestamos y productos en almacen"
            )),
            Card::new(
                text("Ventas"),
                get_total_label(format!(
                    "#{number}: ${money}",
                    number = data.data_stats.sales,
                    money = data.data_stats.total_sales
                ))
            )
            .foot(text("suma de todas las ventas y la ganancia de estas"))
        )
        .spacing(SPACE_ROWS),
        Card::new(
            text("Prestamos"),
            get_total_label(format!(
                "#{number}: ${money}",
                number = data.data_stats.loans,
                money = data.data_stats.total_loans
            ))
        )
        .foot(text("suma de todos los préstamos y pérdidas de estas"))
    )
    .padding(COLUMN_PADDING)
    .spacing(SPACE_COLUMNS)
    .align_items(Alignment::Center)
    .into()
}
