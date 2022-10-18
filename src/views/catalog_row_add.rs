use iced::{Button, Element, Length, Row, Text};

use crate::constants::SIZE_TEXT;
use crate::kinds::AppEvents;
use crate::schemas::catalog::LoadProduct;
use crate::views::fonts;

impl LoadProduct {
    pub fn get_formatted_row(&mut self, id: String) -> Element<AppEvents> {
        Row::new()
            .push(
                Text::new(format!("{}: {}", self.barcode, self.product_name))
                    .size(SIZE_TEXT)
                    .width(Length::FillPortion(6)),
            )
            .push(
                Text::new(&self.amount)
                    .size(SIZE_TEXT)
                    .width(Length::FillPortion(2)),
            )
            .push(
                Text::new(&self.cost)
                    .size(SIZE_TEXT)
                    .width(Length::FillPortion(2)),
            )
            .push(
                Button::new(
                    &mut self.edit_button_state,
                    Text::new('\u{F1F8}'.to_string()).font(fonts::ICONS),
                )
                .on_press(AppEvents::RemoveRecordList(id)),
            )
            .spacing(10)
            .width(iced::Length::Fill)
            .into()
    }
}
