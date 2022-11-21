//! [`iced::Element`]s to be used in the products to buy view

use iced::{
    widget::{column, container, scrollable, text},
    Alignment, Element, Length,
};

use crate::{
    constants::{COLUMN_PADDING, SIZE_TEXT},
    kinds::AppEvents,
    models::catalog::ProductToBuy,
};

/// Show the list of products to be bought
pub fn show_list_products(products: &[ProductToBuy]) -> Element<AppEvents> {
    let mut col = column!()
        .align_items(Alignment::Start)
        .padding(COLUMN_PADDING)
        .spacing(20);
    for product in products {
        col = col.push(text(product.get_formatted_item()).size(SIZE_TEXT));
    }

    container(scrollable(col).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
