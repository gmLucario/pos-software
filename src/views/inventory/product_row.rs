//! Product Row Component
//!
//! Displays a single product as a table row in the inventory view.

use crate::models::Product;
use crate::utils::formatting::format_currency;
use dioxus::prelude::*;

#[component]
pub fn ProductRow(product: Product, on_edit: EventHandler<Product>) -> Element {
    let is_low_stock = product.is_low_stock();
    let stock_style = get_stock_style(is_low_stock);

    rsx! {
        tr {
            style: "border-bottom: 1px solid #e2e8f0; transition: background 0.2s;",
            onmouseenter: move |_| {},

            td {
                style: "padding: 0.75rem; font-weight: 500; font-size: 1rem;",
                "{product.full_name}"
            }
            td {
                style: "padding: 0.75rem; color: #718096; font-family: monospace; font-size: 1rem;",
                "{product.barcode.as_deref().unwrap_or(\"-\")}"
            }
            td {
                style: "padding: 0.75rem; text-align: center; font-weight: 500; font-size: 1rem;",
                "{format_currency(product.user_price)}"
            }
            td {
                style: "padding: 0.75rem; text-align: center; font-size: 1rem; {stock_style}",
                "{product.current_amount:.2}"
            }
            td {
                style: "padding: 0.75rem; text-align: center;",
                { stock_status_badge(is_low_stock) }
            }
            td {
                style: "padding: 0.75rem; text-align: right;",
                button {
                    style: "background: none; border: none; color: #667eea; font-weight: 500; cursor: pointer; padding: 0.25rem 0.5rem; font-size: 0.875rem;",
                    onclick: move |_| on_edit.call(product.clone()),
                    "Edit"
                }
            }
        }
    }
}

/// Helper function to get stock style based on stock level
fn get_stock_style(is_low_stock: bool) -> &'static str {
    if is_low_stock {
        "color: #f56565; font-weight: 600;"
    } else {
        "color: #48bb78;"
    }
}

/// Helper function to render stock status badge
fn stock_status_badge(is_low_stock: bool) -> Element {
    if is_low_stock {
        rsx! {
            span {
                style: "background: #fff5f5; color: #c53030; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.875rem; font-weight: 500;",
                "⚠️ Low Stock"
            }
        }
    } else {
        rsx! {
            span {
                style: "background: #f0fff4; color: #22543d; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.875rem; font-weight: 500;",
                "✓ In Stock"
            }
        }
    }
}
