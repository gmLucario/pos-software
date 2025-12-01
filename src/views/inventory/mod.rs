//! Inventory Module
//!
//! UI components for managing product inventory.

use dioxus::prelude::*;
use crate::handlers::AppState;
use crate::models::Product;
use crate::utils::formatting::format_currency;

#[component]
pub fn InventoryView() -> Element {
    // Get app state from context
    let app_state = use_context::<AppState>();

    let mut search_query = use_signal(String::new);
    let mut show_add_form = use_signal(|| false);
    let mut refresh_trigger = use_signal(|| 0);

    // Load products from database
    let products_resource = use_resource(move || {
        let handler = app_state.inventory_handler.clone();
        async move {
            handler.load_products().await
        }
    });

    // Refresh products when trigger changes
    use_effect(move || {
        let _ = refresh_trigger();
        products_resource.restart();
    });

    rsx! {
        div {
            class: "inventory-view",
            style: "background: white; border-radius: 0.5rem; padding: 1.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",

            // Header
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;",

                h2 {
                    style: "font-size: 1.5rem; font-weight: 600; color: #2d3748; margin: 0;",
                    "📦 Product Inventory"
                }

                div {
                    style: "display: flex; gap: 0.5rem;",

                    button {
                        style: "background: #48bb78; color: white; padding: 0.75rem 1.5rem; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer; transition: background 0.2s;",
                        onclick: move |_| refresh_trigger.set(refresh_trigger() + 1),
                        "🔄 Refresh"
                    }

                    button {
                        style: "background: #667eea; color: white; padding: 0.75rem 1.5rem; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer; transition: background 0.2s;",
                        onclick: move |_| show_add_form.set(!show_add_form()),
                        "+ Add Product"
                    }
                }
            }

            // Search bar
            div {
                style: "margin-bottom: 1.5rem;",

                input {
                    r#type: "text",
                    placeholder: "🔍 Search by name or barcode...",
                    value: "{search_query}",
                    oninput: move |evt| search_query.set(evt.value().clone()),
                    style: "width: 100%; padding: 0.75rem; border: 2px solid #e2e8f0; border-radius: 0.5rem; font-size: 1rem; box-sizing: border-box;",
                }
            }

            // Content based on loading state
            match &*products_resource.read_unchecked() {
                Some(Ok(products)) => {
                    // Filter products based on search
                    let filtered_products: Vec<Product> = products.iter()
                        .filter(|p| {
                            let query = search_query.read().to_lowercase();
                            if query.is_empty() {
                                return true;
                            }
                            p.full_name.to_lowercase().contains(&query) ||
                            p.barcode.as_ref().is_some_and(|b| b.contains(&query))
                        })
                        .cloned()
                        .collect();

                    let low_stock_count = filtered_products.iter().filter(|p| p.is_low_stock()).count();
                    let total_value: rust_decimal::Decimal = filtered_products.iter()
                        .map(|p| p.user_price * rust_decimal::Decimal::from_f64_retain(p.current_amount).unwrap_or_default())
                        .sum();

                    rsx! {
                        // Products table
                        div {
                            style: "overflow-x: auto;",

                            table {
                                style: "width: 100%; border-collapse: collapse;",

                                thead {
                                    tr {
                                        style: "background: #f7fafc; border-bottom: 2px solid #e2e8f0;",

                                        th { style: "padding: 0.75rem; text-align: left; font-weight: 600; color: #4a5568;", "Product Name" }
                                        th { style: "padding: 0.75rem; text-align: left; font-weight: 600; color: #4a5568;", "Barcode" }
                                        th { style: "padding: 0.75rem; text-align: center; font-weight: 600; color: #4a5568;", "Price" }
                                        th { style: "padding: 0.75rem; text-align: center; font-weight: 600; color: #4a5568;", "Stock" }
                                        th { style: "padding: 0.75rem; text-align: center; font-weight: 600; color: #4a5568;", "Status" }
                                    }
                                }

                                tbody {
                                    if filtered_products.is_empty() {
                                        tr {
                                            td {
                                                colspan: "5",
                                                style: "padding: 2rem; text-align: center; color: #718096;",
                                                "No products found. Add some products to get started!"
                                            }
                                        }
                                    } else {
                                        for product in filtered_products.iter() {
                                            ProductRow { product: product.clone() }
                                        }
                                    }
                                }
                            }
                        }

                        // Stats summary
                        div {
                            style: "margin-top: 1.5rem; padding-top: 1.5rem; border-top: 2px solid #e2e8f0; display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;",

                            StatCard {
                                label: "Total Products",
                                value: format!("{}", filtered_products.len()),
                                color: "#667eea",
                            }

                            StatCard {
                                label: "Low Stock Items",
                                value: format!("{}", low_stock_count),
                                color: "#f56565",
                            }

                            StatCard {
                                label: "Total Value",
                                value: format_currency(total_value),
                                color: "#48bb78",
                            }
                        }
                    }
                },
                Some(Err(err)) => rsx! {
                    div {
                        style: "padding: 2rem; text-align: center; color: #e53e3e; background: #fff5f5; border-radius: 0.5rem;",
                        "❌ Error loading products: {err}"
                    }
                },
                None => rsx! {
                    div {
                        style: "padding: 2rem; text-align: center; color: #718096;",
                        "⏳ Loading products..."
                    }
                }
            }
        }
    }
}

#[component]
fn ProductRow(product: Product) -> Element {
    let is_low_stock = product.is_low_stock();

    let stock_style = if is_low_stock {
        "color: #f56565; font-weight: 600;"
    } else {
        "color: #48bb78;"
    };

    rsx! {
        tr {
            style: "border-bottom: 1px solid #e2e8f0; transition: background 0.2s;",
            onmouseenter: move |_| {},

            td {
                style: "padding: 0.75rem; font-weight: 500;",
                "{product.full_name}"
            }
            td {
                style: "padding: 0.75rem; color: #718096; font-family: monospace;",
                "{product.barcode.as_deref().unwrap_or(\"-\")}"
            }
            td {
                style: "padding: 0.75rem; text-align: center; font-weight: 500;",
                "{format_currency(product.user_price)}"
            }
            td {
                style: "padding: 0.75rem; text-align: center; {stock_style}",
                "{product.current_amount:.2}"
            }
            td {
                style: "padding: 0.75rem; text-align: center;",
                if is_low_stock {
                    span {
                        style: "background: #fff5f5; color: #c53030; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.875rem; font-weight: 500;",
                        "⚠️ Low Stock"
                    }
                } else {
                    span {
                        style: "background: #f0fff4; color: #22543d; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.875rem; font-weight: 500;",
                        "✓ In Stock"
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(label: String, value: String, color: String) -> Element {
    rsx! {
        div {
            style: "background: #f7fafc; padding: 1rem; border-radius: 0.5rem; border-left: 4px solid {color};",

            div {
                style: "font-size: 0.875rem; color: #718096; margin-bottom: 0.5rem;",
                "{label}"
            }
            div {
                style: "font-size: 1.5rem; font-weight: 700; color: {color};",
                "{value}"
            }
        }
    }
}
