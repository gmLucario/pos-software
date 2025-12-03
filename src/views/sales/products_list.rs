//! Products List Component
//!
//! Displays a grid of available products with search functionality.

use crate::models::Product;
use dioxus::prelude::*;

use super::{CartItem, ProductCard};

#[component]
pub fn ProductsList(
    products: ReadSignal<Vec<Product>>,
    cart_items: ReadSignal<Vec<CartItem>>,
    search_query: ReadSignal<String>,
    on_add: EventHandler<Product>,
) -> Element {
    // Filter products based on search query - use memo to avoid re-filtering
    let filtered_products = use_memo(move || {
        let query = search_query.read();
        let products_read = products.read();

        if query.is_empty() {
            Vec::new()
        } else {
            let query_lower = query.to_lowercase();
            products_read
                .iter()
                .filter(|p| {
                    p.full_name.to_lowercase().contains(&query_lower)
                        || p.barcode.as_ref().is_some_and(|b| b.contains(&query_lower))
                })
                .take(5)
                .cloned()
                .collect()
        }
    });

    rsx! {
        div {
            style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 1rem; max-height: 600px; overflow-y: auto;",

            if !search_query.read().is_empty() && filtered_products.read().is_empty() {
                div {
                    style: "grid-column: 1 / -1; padding: 2rem; text-align: center; color: #718096;",
                    "No products found"
                }
            } else {
                for product in filtered_products.read().iter() {
                    ProductCard {
                        product: product.clone(),
                        cart_items: cart_items,
                        on_add: move |p: Product| on_add.call(p),
                    }
                }
            }
        }
    }
}
