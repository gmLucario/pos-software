//! Handle logic to link [`crate::views::to_buy`]
//! module with the [`crate::data::product_repo::ProductRepo`]

use iced::scrollable;

use crate::models::catalog::ProductsToBuy;

/// Controller links [`crate::views::to_buy`] module with the [`crate::data::product_repo::ProductRepo`]
/// and perform the logic of products to be bought to fill the stock
#[derive(Default)]
pub struct ToBuy {
    /// List products to be bought
    pub products: Vec<ProductsToBuy>,
    /// Iced scrollable state
    pub scroll_list_state: scrollable::State,
}
