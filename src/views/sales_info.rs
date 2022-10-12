use iced::{
    widget::{Container, Text},
    Element, Length,
};

use crate::constants::{SALES_INFO_BTN_MSG, SIZE_TEXT};
use crate::kinds::AppEvents;

#[derive(Default)]
pub struct SalesInfo {}

impl SalesInfo {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&mut self) -> Element<AppEvents> {
        let label = Text::new(SALES_INFO_BTN_MSG).size(SIZE_TEXT);

        Container::new(label)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}