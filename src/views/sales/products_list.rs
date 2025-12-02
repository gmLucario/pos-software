//! Products List Component
//!
//! Displays a grid of available products with search functionality.

use crate::models::Product;
use dioxus::prelude::*;

use super::{CartItem, ProductCard};

#[component]
pub fn ProductsList(
    products: Vec<Product>,
    cart_items: Vec<CartItem>,
    search_query: String,
    on_add: EventHandler<Product>,
) -> Element {
    // Filter products based on search query
    let filtered_products: Vec<Product> = products
        .iter()
        .filter(|p| {
            if search_query.is_empty() {
                return true;
            }
            let query = search_query.to_lowercase();
            p.full_name.to_lowercase().contains(&query)
                || p.barcode.as_ref().is_some_and(|b| b.contains(&query))
        })
        .cloned()
        .collect();

    rsx! {
        div {
            style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 1rem; max-height: 600px; overflow-y: auto;",

            if filtered_products.is_empty() {
                div {
                    style: "grid-column: 1 / -1; padding: 2rem; text-align: center; color: #718096;",
                    "No products found"
                }
            } else {
                for product in filtered_products {
                    ProductCard {
                        product: product.clone(),
                        cart_items: cart_items.clone(),
                        on_add: move |p: Product| on_add.call(p),
                    }
                }
            }
        }
    }
}
