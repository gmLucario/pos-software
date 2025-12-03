//! Inventory Module
//!
//! UI components for managing product inventory.

mod helpers;
mod pagination_nav;
mod product_form;
mod product_row;
mod products_table;
mod stat_card;
mod stats_summary;

use crate::handlers::AppState;
use crate::models::Product;
use dioxus::prelude::*;
use helpers::{calculate_total_pages, InventoryStats};
use pagination_nav::PaginationNav;
use product_form::ProductForm;
use products_table::ProductsTable;
use stats_summary::StatsSummary;

const PAGE_SIZE: i64 = 10;

#[component]
pub fn InventoryView() -> Element {
    let app_state = use_context::<AppState>();

    // State management
    let mut search_query = use_signal(String::new);
    let mut show_add_form = use_signal(|| false);
    let mut editing_product = use_signal(|| None::<Product>);
    let mut refresh_trigger = use_signal(|| 0);
    let mut current_page = use_signal(|| 1i64);

    // Load products with pagination (always paginated, whether searching or not)
    let mut products_resource = use_resource({
        let handler = app_state.inventory_handler.clone();

        move || {
            let handler = handler.clone();
            let page = current_page();
            let query = search_query();

            async move {
                handler
                    .search_products_paginated(query, page, PAGE_SIZE)
                    .await
                    .map(|paginated| {
                        (
                            paginated.items,
                            Some((paginated.total_count, paginated.page)),
                        )
                    })
            }
        }
    });

    // Reset to page 1 when search query changes
    use_effect(move || {
        let _ = search_query();
        current_page.set(1);
    });

    // Refresh products when trigger changes
    use_effect(move || {
        let _ = refresh_trigger();
        products_resource.restart();
    });

    // Clone handlers for use in closures
    let save_handler = app_state.inventory_handler.clone();
    let delete_handler = app_state.inventory_handler.clone();

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

                button {
                    style: "background: #667eea; color: white; padding: 0.75rem 1.5rem; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer; transition: background 0.2s; font-size: 1rem;",
                    onclick: move |_| {
                        editing_product.set(None);
                        show_add_form.set(true);
                    },
                    "+ Add Product"
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

            // Content
            {
                match &*products_resource.read_unchecked() {
                    Some(Ok((products, pagination_info))) => {
                        let is_searching = !search_query().trim().is_empty();
                        let total_count = pagination_info.map(|(count, _)| count).unwrap_or(products.len() as i64);
                        let total_pages = calculate_total_pages(total_count, PAGE_SIZE);
                        let stats = InventoryStats::calculate(products, total_count);

                        rsx! {
                            ProductsTable {
                                products: products.clone(),
                                is_search_mode: is_searching,
                                on_edit: move |p| {
                                    editing_product.set(Some(p));
                                    show_add_form.set(true);
                                }
                            }

                            PaginationNav {
                                current_page,
                                total_pages,
                            }

                            StatsSummary {
                                stats: stats,
                                is_search_mode: is_searching,
                            }

                        }
                    }
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
                    },
                }
            }

            if show_add_form() {
                ProductForm {
                    on_close: move |_| {
                        show_add_form.set(false);
                        editing_product.set(None);
                    },
                    initial_product: editing_product(),
                    on_save: move |input| {
                        let handler = save_handler.clone();
                        let is_edit = editing_product().is_some();
                        let edit_id = editing_product().as_ref().map(|p| p.id.clone());

                        spawn(async move {
                            let result = if is_edit {
                                handler.update_product(edit_id.unwrap(), input).await
                            } else {
                                handler.create_product(input).await
                            };

                            if result.is_ok() {
                                show_add_form.set(false);
                                editing_product.set(None);
                                refresh_trigger.set(refresh_trigger() + 1);
                            }
                        });
                    },
                    on_delete: move |id| {
                        let handler = delete_handler.clone();
                        spawn(async move {
                            if handler.delete_product(id).await.is_ok() {
                                show_add_form.set(false);
                                editing_product.set(None);
                                refresh_trigger.set(refresh_trigger() + 1);
                            }
                        });
                    }
                }
            }
        }
    }
}
