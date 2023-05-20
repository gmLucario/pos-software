//! [`iced::Element`]s to be used in the To Buy to buy view

use iced::{
    widget::{column, container, scrollable, text, text_input, Column},
    Element, Length,
};

use crate::{
    constants::{COLUMN_PADDING, SIZE_TEXT, SIZE_TEXT_INPUT, SPACE_COLUMNS},
    events::AppEvent,
    kinds::{TextInput, View},
    models::catalog::ProductToBuy,
};

/// Show products in the catalog
pub fn show_list_products<'a>(
    products: &[ProductToBuy],
    text_input_value: &str,
) -> Element<'a, AppEvent> {
    let products: Vec<Element<AppEvent>> = products
        .iter()
        .map(|pr| text(pr.get_formatted_item()).size(SIZE_TEXT).into())
        .collect();

    let col = Column::with_children(products)
        .spacing(SPACE_COLUMNS)
        .width(Length::Fill);

    column!(
        text_input("", text_input_value)
            .on_input(|input_value| {
                AppEvent::TextInputChanged(input_value, TextInput::ToBuyProductLike)
            })
            .on_submit(AppEvent::ChangeView(View::ToBuy))
            .size(SIZE_TEXT_INPUT),
        container(scrollable(col).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .padding(COLUMN_PADDING)
    .spacing(SPACE_COLUMNS)
    .into()
}
