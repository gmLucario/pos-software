use iced::{
    widget::{column, container, scrollable, text},
    Alignment, Element, Length,
};

use crate::{constants::SIZE_TEXT, kinds::AppEvents, models::catalog::ProductToBuy};

pub fn show_list_products(products: &[ProductToBuy]) -> Element<AppEvents> {
    let mut col = column!()
        .align_items(Alignment::Start)
        .padding(20)
        .spacing(20);
    for product in products.iter() {
        col = col.push(text(product.get_formatted_item()).size(SIZE_TEXT));
    }

    container(scrollable(col).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
