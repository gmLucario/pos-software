//! Product Card Component
//!
//! Displays a product card in the sales view with product information and add to cart functionality.

use crate::models::Product;
use crate::utils::formatting::format_currency;
use dioxus::prelude::*;

use super::{get_unit_abbreviation, CartItem};

#[component]
pub fn ProductCard(
    product: Product,
    cart_items: ReadSignal<Vec<CartItem>>,
    on_add: EventHandler<Product>,
) -> Element {
    // Calculate remaining stock (current stock - quantity in cart)
    let quantity_in_cart = cart_items
        .read()
        .iter()
        .find(|item| item.product.id == product.id)
        .map(|item| item.quantity)
        .unwrap_or(0.0);

    let remaining_stock = product.current_amount - quantity_in_cart;
    let is_low_stock = remaining_stock <= product.min_amount;
    let unit_abbr = get_unit_abbreviation(product.unit_measurement_id);

    rsx! {
        div {
            style: "border: 2px solid #e2e8f0; border-radius: 0.5rem; padding: 1rem; cursor: pointer; transition: all 0.2s; background: white;",
            onclick: move |_| on_add.call(product.clone()),

            div {
                style: "font-weight: 600; margin-bottom: 0.5rem; color: #2d3748; font-size: 1rem;",
                "{product.full_name}"
            }

            div {
                style: "font-size: 1.25rem; font-weight: 700; color: #667eea; margin-bottom: 0.5rem;",
                "{format_currency(product.user_price)}"
            }

            div {
                style: "font-size: 0.875rem; color: #718096;",
                "Stock: {remaining_stock:.2} {unit_abbr}"
            }

            if is_low_stock {
                div {
                    style: "margin-top: 0.5rem; background: #fff5f5; color: #c53030; padding: 0.25rem 0.5rem; border-radius: 0.25rem; font-size: 0.75rem; text-align: center;",
                    "⚠️ Low Stock"
                }
            }
        }
    }
}
