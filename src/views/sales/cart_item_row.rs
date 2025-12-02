//! Cart Item Row Component
//!
//! Displays a single cart item with product details, quantity, and remove button.

use crate::utils::formatting::format_currency;
use dioxus::prelude::*;
use rust_decimal::Decimal;

use super::CartItem;

#[component]
pub fn CartItemRow(item: CartItem, on_remove: EventHandler<String>) -> Element {
    let subtotal =
        item.product.user_price * Decimal::from_f64_retain(item.quantity).unwrap_or_default();

    rsx! {
        div {
            style: "padding: 0.75rem; border-bottom: 1px solid #e2e8f0; display: flex; justify-content: space-between; align-items: center;",

            div {
                style: "flex: 1;",
                div {
                    style: "font-weight: 500; color: #2d3748;",
                    "{item.product.full_name}"
                }
                div {
                    style: "font-size: 0.875rem; color: #718096;",
                    "{item.quantity} × {format_currency(item.product.user_price)}"
                }
            }

            div {
                style: "display: flex; align-items: center; gap: 1rem;",
                div {
                    style: "font-weight: 600; color: #667eea;",
                    "{format_currency(subtotal)}"
                }
                button {
                    style: "background: #f56565; color: white; border: none; border-radius: 0.25rem; padding: 0.25rem 0.5rem; cursor: pointer; font-size: 0.875rem;",
                    onclick: move |_| on_remove.call(item.product.id.clone()),
                    "✕"
                }
            }
        }
    }
}
