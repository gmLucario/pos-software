use iced::{
    scrollable,
    widget::{Container, Text},
    Alignment, Column, Element, Length, Scrollable,
};

use crate::constants::SIZE_TEXT;
use crate::kinds::AppEvents;
use crate::schemas::catalog::ProductsToBy;

#[derive(Default)]
pub struct ToBuy {
    pub products: Vec<ProductsToBy>,
    pub scroll_list_state: scrollable::State,
}

impl ToBuy {
    pub fn new() -> Self {
        Self {
            products: vec![],
            scroll_list_state: scrollable::State::new(),
        }
    }

    pub fn view(&mut self, products: &Vec<ProductsToBy>) -> Element<AppEvents> {
        let mut col = Column::new()
            .align_items(Alignment::Start)
            .padding(20)
            .spacing(20);

        for product in products {
            col = col.push(
                Text::new(format!(
                    "{product_name}: {amount_to_buy}",
                    product_name = product.product_name,
                    amount_to_buy = product.amount_to_buy
                ))
                .size(SIZE_TEXT),
            );
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
