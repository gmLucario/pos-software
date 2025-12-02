//! Products Table Component
//!
//! Displays the table of products with headers and rows.

use crate::models::Product;
use crate::views::inventory::product_row::ProductRow;
use dioxus::prelude::*;

#[component]
pub fn ProductsTable(
    products: Vec<Product>,
    is_search_mode: bool,
    on_edit: EventHandler<Product>,
) -> Element {
    rsx! {
        div {
            style: "overflow-x: auto;",

            table {
                style: "width: 100%; border-collapse: collapse;",

                thead {
                    { table_header() }
                }

                tbody {
                    if products.is_empty() {
                        { empty_row(is_search_mode) }
                    } else {
                        for product in products.iter() {
                            ProductRow {
                                product: product.clone(),
                                on_edit: move |p| on_edit.call(p),
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Render table header
fn table_header() -> Element {
    rsx! {
        tr {
            style: "background: #f7fafc; border-bottom: 2px solid #e2e8f0;",

            th { style: HEADER_STYLE, "Product Name" }
            th { style: HEADER_STYLE, "Barcode" }
            th { style: "{HEADER_STYLE} text-align: center;", "Price" }
            th { style: "{HEADER_STYLE} text-align: center;", "Stock" }
            th { style: "{HEADER_STYLE} text-align: center;", "Status" }
            th { style: "{HEADER_STYLE} text-align: right;", "Actions" }
        }
    }
}

/// Render empty state row
fn empty_row(is_search_mode: bool) -> Element {
    let message = if is_search_mode {
        "No products found matching your search."
    } else {
        "No products found. Add some products to get started!"
    };

    rsx! {
        tr {
            td {
                colspan: "6",
                style: "padding: 2rem; text-align: center; color: #718096;",
                "{message}"
            }
        }
    }
}

const HEADER_STYLE: &str = "padding: 0.75rem; text-align: left; font-weight: 600; color: #4a5568; font-size: 0.875rem; text-transform: uppercase; letter-spacing: 0.05em;";
