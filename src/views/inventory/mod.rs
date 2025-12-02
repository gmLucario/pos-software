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
use crate::models::{Product, ProductInput};
use dioxus::prelude::*;
use helpers::{calculate_total_pages, is_search_mode, InventoryStats};
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

    // Load products with pagination or search
    let mut products_resource = use_resource({
        let handler = app_state.inventory_handler.clone();
        let search_handler = app_state.inventory_handler.clone();

        move || {
            let handler = handler.clone();
            let search_h = search_handler.clone();
            let page = current_page();
            let query = search_query();

            async move {
                if query.trim().is_empty() {
                    handler
                        .load_products_paginated(page, PAGE_SIZE)
                        .await
                        .map(|paginated| (paginated.items, Some((paginated.total_count, paginated.page))))
                } else {
                    search_h
                        .search_products(query)
                        .await
                        .map(|products| (products, None))
                }
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

    rsx! {
        div {
            class: "inventory-view",
            style: "background: white; border-radius: 0.5rem; padding: 1.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",

            { render_header(show_add_form, editing_product) }
            { render_search_bar(search_query) }
            { render_content(products_resource, current_page, show_add_form, editing_product) }

            if show_add_form() {
                ProductForm {
                    on_close: move |_| {
                        show_add_form.set(false);
                        editing_product.set(None);
                    },
                    initial_product: editing_product(),
                    on_save: move |input| {
                        handle_save(
                            input,
                            &editing_product(),
                            app_state.inventory_handler.clone(),
                            show_add_form,
                            editing_product,
                            refresh_trigger,
                        );
                    },
                    on_delete: move |id| {
                        handle_delete(
                            id,
                            app_state.inventory_handler.clone(),
                            show_add_form,
                            editing_product,
                            refresh_trigger,
                        );
                    }
                }
            }
        }
    }
}

/// Render the header with title and add button
fn render_header(
    show_add_form: Signal<bool>,
    editing_product: Signal<Option<Product>>,
) -> Element {
    rsx! {
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
    }
}

/// Render the search bar
fn render_search_bar(search_query: Signal<String>) -> Element {
    rsx! {
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
    }
}

/// Render the main content area
fn render_content(
    products_resource: Resource<Result<(Vec<Product>, Option<(i64, i64)>), String>>,
    current_page: Signal<i64>,
    show_add_form: Signal<bool>,
    editing_product: Signal<Option<Product>>,
) -> Element {
    match &*products_resource.read_unchecked() {
        Some(Ok((products, pagination_info))) => {
            let is_search = is_search_mode(pagination_info);
            let total_count = pagination_info.map(|(count, _)| count).unwrap_or(products.len() as i64);
            let total_pages = calculate_total_pages(total_count, PAGE_SIZE);
            let stats = InventoryStats::calculate(products, total_count);

            rsx! {
                ProductsTable {
                    products: products.clone(),
                    is_search_mode: is_search,
                    on_edit: move |p| {
                        editing_product.set(Some(p));
                        show_add_form.set(true);
                    }
                }

                StatsSummary {
                    stats: stats,
                    is_search_mode: is_search,
                }

                if !is_search {
                    PaginationNav {
                        current_page,
                        total_pages,
                    }
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

/// Handle product save (create or update)
fn handle_save(
    input: ProductInput,
    editing: &Option<Product>,
    handler: crate::handlers::InventoryHandler,
    show_form: Signal<bool>,
    editing_signal: Signal<Option<Product>>,
    refresh: Signal<i32>,
) {
    let is_edit = editing.is_some();
    let edit_id = editing.as_ref().map(|p| p.id.clone());
    let create_handler = handler.clone();
    let update_handler = handler;

    spawn(async move {
        let result = if is_edit {
            update_handler.update_product(edit_id.unwrap(), input).await
        } else {
            create_handler.create_product(input).await
        };

        if result.is_ok() {
            show_form.set(false);
            editing_signal.set(None);
            refresh.set(refresh() + 1);
        }
    });
}

/// Handle product deletion
fn handle_delete(
    id: String,
    handler: crate::handlers::InventoryHandler,
    show_form: Signal<bool>,
    editing: Signal<Option<Product>>,
    refresh: Signal<i32>,
) {
    spawn(async move {
        if handler.delete_product(id).await.is_ok() {
            show_form.set(false);
            editing.set(None);
            refresh.set(refresh() + 1);
        }
    });
}
