//! Handle logic to link [`crate::views::to_buy`]
//! module with the [`crate::repo::product_repo`]

use crate::models::catalog::ProductToBuy;

/// Controller links [`crate::views::to_buy`] module with the [`crate::repo::product_repo`]
/// and perform the logic of products to be bought to fill the stock
#[derive(Default)]
pub struct ToBuy {
    /// Data to be used to search the products
    pub product_name: String,
    /// List products to be bought
    pub products: Vec<ProductToBuy>,
}
