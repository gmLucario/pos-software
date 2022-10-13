use iced::{
    widget::{Container, Text},
    Alignment, Column, Element, Length, Scrollable,
};

use crate::constants::SIZE_TEXT;
use crate::kinds::AppEvents;

use crate::controllers::to_buy::ToBuy;

impl ToBuy {
    pub fn view(&mut self) -> Element<AppEvents> {
        let mut col = Column::new()
            .align_items(Alignment::Start)
            .padding(20)
            .spacing(20);

        for product in self.products.iter() {
            col = col.push(Text::new(product.get_formatted_item()).size(SIZE_TEXT));
        }

        let scroll_row = Scrollable::new(&mut self.scroll_list_state)
            .push(col)
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(scroll_row)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
