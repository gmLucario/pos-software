use iced::{
    widget::{container, text},
    Element, Length,
};

use crate::{
    constants::{SALES_INFO_BTN_MSG, SIZE_TEXT},
    kinds::AppEvents,
};

pub fn view() -> Element<'static, AppEvents> {
    container(text(SALES_INFO_BTN_MSG).size(SIZE_TEXT))
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
