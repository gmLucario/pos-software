//! Inventory Module
//!
//! UI components for managing product inventory.

use crate::handlers::AppState;
use crate::models::{Product, ProductInput};
use crate::utils::formatting::format_currency;
use dioxus::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;

#[component]
pub fn InventoryView() -> Element {
    // Get app state from context
    let app_state = use_context::<AppState>();

    let mut search_query = use_signal(String::new);
    let mut show_add_form = use_signal(|| false);
    let mut editing_product = use_signal(|| None::<Product>);
    let mut refresh_trigger = use_signal(|| 0);
    let mut current_page = use_signal(|| 1i64);
    let page_size = 10i64; // Items per page

    // Load products from database with pagination or search
    let products_handler = app_state.inventory_handler.clone();
    let search_handler = app_state.inventory_handler.clone();

    // Use pagination when no search, use search when there's a query
    let mut products_resource = use_resource(move || {
        let handler = products_handler.clone();
        let search_h = search_handler.clone();
        let page = current_page();
        let query = search_query();

        async move {
            if query.trim().is_empty() {
                // No search query - use pagination
                handler.load_products_paginated(page, page_size).await.map(|paginated| {
                    (paginated.items, Some((paginated.total_count, paginated.page)))
                })
            } else {
                // Search query present - return all matching products
                search_h.search_products(query).await.map(|products| {
                    (products, None)
                })
            }
        }
    });

    let create_handler = app_state.inventory_handler.clone();
    let update_handler = app_state.inventory_handler.clone();
    let delete_handler = app_state.inventory_handler.clone();

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
                        style: "background: #667eea; color: white; padding: 0.75rem 1.5rem; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer; transition: background 0.2s; font-size: 1rem;",
                        onclick: move |_| {
                            editing_product.set(None);
                            show_add_form.set(true);
                        },
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
                Some(Ok((products, pagination_info))) => {
                    // Determine if we're in search mode or pagination mode
                    let is_search_mode = pagination_info.is_none();
                    let total_count = if let Some((count, _)) = pagination_info {
                        *count
                    } else {
                        products.len() as i64
                    };
                    let total_pages = if let Some((count, _)) = pagination_info {
                        ((*count as f64) / (page_size as f64)).ceil() as i64
                    } else {
                        1
                    };

                    let low_stock_count = products.iter().filter(|p| p.is_low_stock()).count();
                    let total_value: rust_decimal::Decimal = products.iter()
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

                                        th { style: "padding: 0.75rem; text-align: left; font-weight: 600; color: #4a5568; font-size: 0.875rem; text-transform: uppercase; letter-spacing: 0.05em;", "Product Name" }
                                        th { style: "padding: 0.75rem; text-align: left; font-weight: 600; color: #4a5568; font-size: 0.875rem; text-transform: uppercase; letter-spacing: 0.05em;", "Barcode" }
                                        th { style: "padding: 0.75rem; text-align: center; font-weight: 600; color: #4a5568; font-size: 0.875rem; text-transform: uppercase; letter-spacing: 0.05em;", "Price" }
                                        th { style: "padding: 0.75rem; text-align: center; font-weight: 600; color: #4a5568; font-size: 0.875rem; text-transform: uppercase; letter-spacing: 0.05em;", "Stock" }
                                        th { style: "padding: 0.75rem; text-align: center; font-weight: 600; color: #4a5568; font-size: 0.875rem; text-transform: uppercase; letter-spacing: 0.05em;", "Status" }
                                        th { style: "padding: 0.75rem; text-align: right; font-weight: 600; color: #4a5568; font-size: 0.875rem; text-transform: uppercase; letter-spacing: 0.05em;", "Actions" }
                                    }
                                }

                                tbody {
                                    if products.is_empty() {
                                        tr {
                                            td {
                                                colspan: "5",
                                                style: "padding: 2rem; text-align: center; color: #718096;",
                                                if is_search_mode {
                                                    "No products found matching your search."
                                                } else {
                                                    "No products found. Add some products to get started!"
                                                }
                                            }
                                        }
                                    } else {
                                        for product in products.iter() {
                                            ProductRow {
                                                product: product.clone(),
                                                on_edit: move |p| {
                                                    editing_product.set(Some(p));
                                                    show_add_form.set(true);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Stats summary
                        div {
                            style: "margin-top: 1.5rem; padding-top: 1.5rem; border-top: 2px solid #e2e8f0; display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;",

                            StatCard {
                                label: if is_search_mode { "Matching Products" } else { "Total Products" },
                                value: format!("{}", total_count),
                                color: "#667eea",
                            }

                            StatCard {
                                label: "Low Stock Items",
                                value: format!("{}", low_stock_count),
                                color: "#f56565",
                            }

                            StatCard {
                                label: if is_search_mode { "Search Value" } else { "Total Value" },
                                value: format_currency(total_value),
                                color: "#48bb78",
                            }
                        }

                        // Pagination controls (only show when not searching)
                        if !is_search_mode && total_pages > 1 {
                            div {
                                style: "margin-top: 1.5rem; display: flex; justify-content: center; align-items: center; gap: 1rem;",

                                button {
                                    style: format!(
                                        "padding: 0.5rem 1rem; border: 1px solid #e2e8f0; background: {}; border-radius: 0.5rem; cursor: {}; font-size: 0.875rem; font-weight: 500; color: {};",
                                        if current_page() > 1 { "white" } else { "#f7fafc" },
                                        if current_page() > 1 { "pointer" } else { "not-allowed" },
                                        if current_page() > 1 { "#4a5568" } else { "#cbd5e0" }
                                    ),
                                    disabled: current_page() <= 1,
                                    onclick: move |_| {
                                        if current_page() > 1 {
                                            current_page.set(current_page() - 1);
                                        }
                                    },
                                    "← Previous"
                                }

                                span {
                                    style: "font-size: 0.875rem; color: #4a5568;",
                                    "Page {current_page()} of {total_pages}"
                                }

                                button {
                                    style: format!(
                                        "padding: 0.5rem 1rem; border: 1px solid #e2e8f0; background: {}; border-radius: 0.5rem; cursor: {}; font-size: 0.875rem; font-weight: 500; color: {};",
                                        if current_page() < total_pages { "white" } else { "#f7fafc" },
                                        if current_page() < total_pages { "pointer" } else { "not-allowed" },
                                        if current_page() < total_pages { "#4a5568" } else { "#cbd5e0" }
                                    ),
                                    disabled: current_page() >= total_pages,
                                    onclick: move |_| {
                                        if current_page() < total_pages {
                                            current_page.set(current_page() + 1);
                                        }
                                    },
                                    "Next →"
                                }
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


            if show_add_form() {


                ProductForm {
                    on_close: move |_| {
                        show_add_form.set(false);
                        editing_product.set(None);
                    },
                    initial_product: editing_product(),
                    on_save: move |input| {
                        let handler = create_handler.clone();
                        let update_handler = update_handler.clone();
                        let is_edit = editing_product().is_some();
                        let edit_id = editing_product().as_ref().map(|p| p.id.clone());

                        spawn(async move {
                            let result = if is_edit {
                                update_handler.update_product(edit_id.unwrap(), input).await
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

#[component]
fn ProductRow(product: Product, on_edit: EventHandler<Product>) -> Element {
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

#[component]
fn ProductForm(
    on_close: EventHandler<()>,
    on_save: EventHandler<ProductInput>,
    on_delete: EventHandler<String>,
    initial_product: Option<Product>,
) -> Element {
    let mut full_name = use_signal(|| {
        initial_product
            .as_ref()
            .map(|p| p.full_name.clone())
            .unwrap_or_default()
    });
    let mut barcode = use_signal(|| {
        initial_product
            .as_ref()
            .and_then(|p| p.barcode.clone())
            .unwrap_or_default()
    });
    let mut price = use_signal(|| {
        initial_product
            .as_ref()
            .map(|p| p.user_price.to_string())
            .unwrap_or_default()
    });
    let mut cost = use_signal(|| {
        initial_product
            .as_ref()
            .and_then(|p| p.cost_price.map(|c| c.to_string()))
            .unwrap_or_default()
    });
    let mut stock = use_signal(|| {
        initial_product
            .as_ref()
            .map(|p| p.current_amount.to_string())
            .unwrap_or_default()
    });
    let mut min_stock = use_signal(|| {
        initial_product
            .as_ref()
            .map(|p| p.min_amount.to_string())
            .unwrap_or_default()
    });
    let mut unit_id = use_signal(|| {
        initial_product
            .as_ref()
            .map(|p| p.unit_measurement_id)
            .unwrap_or(3)
    }); // Default to Unit (3)
    let mut error_msg = use_signal(String::new);

    let units_resource = use_resource(move || async move {
        let app_state = use_context::<AppState>();
        app_state.inventory_handler.get_units().await
    });

    let handle_submit = move |_| {
        let name = full_name();
        if name.trim().is_empty() {
            error_msg.set("Product name is required".to_string());
            return;
        }

        let user_price = match Decimal::from_str(&price()) {
            Ok(p) => p,
            Err(_) => {
                error_msg.set("Invalid price".to_string());
                return;
            }
        };

        let cost_price = if cost().is_empty() {
            None
        } else {
            match Decimal::from_str(&cost()) {
                Ok(c) => Some(c),
                Err(_) => {
                    error_msg.set("Invalid cost price".to_string());
                    return;
                }
            }
        };

        let current_amount = stock().parse::<f64>().unwrap_or(0.0);
        let min_amount = min_stock().parse::<f64>().unwrap_or(0.0);

        on_save.call(ProductInput {
            full_name: name,
            barcode: if barcode().is_empty() {
                None
            } else {
                Some(barcode())
            },
            user_price,
            cost_price,
            current_amount,
            min_amount,
            unit_measurement_id: unit_id(),
        });
    };

    let title = if initial_product.is_some() {
        "Edit Product"
    } else {
        "Add New Product"
    };
    let save_label = if initial_product.is_some() {
        "Update Product"
    } else {
        "Save Product"
    };

    rsx! {
        div {
            style: "position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.5); display: flex; justify-content: center; align-items: center; z-index: 1000;",
            onclick: move |_| on_close.call(()),

            div {
                style: "background: white; padding: 2rem; border-radius: 0.5rem; width: 500px; max-width: 90%; max-height: 90vh; overflow-y: auto;",
                onclick: move |evt| evt.stop_propagation(),

                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;",
                    h3 { style: "margin: 0; font-size: 1.25rem;", "{title}" }

                    if let Some(product) = initial_product {
                        button {
                            style: "background: #e53e3e; color: white; border: none; padding: 0.5rem 1rem; border-radius: 0.25rem; cursor: pointer; font-size: 0.875rem; font-weight: 500;",
                            onclick: move |_| on_delete.call(product.id.clone()),
                            "Delete Product"
                        }
                    }
                }

                if !error_msg().is_empty() {
                    div {
                        style: "background: #fff5f5; color: #c53030; padding: 0.75rem; border-radius: 0.25rem; margin-bottom: 1rem;",
                        "{error_msg}"
                    }
                }

                div {
                    style: "margin-bottom: 1rem;",
                    label { style: "display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem; color: #4a5568;", "Product Name *" }
                    input {
                        r#type: "text",
                        style: "width: 100%; padding: 0.625rem; border: 1px solid #e2e8f0; border-radius: 0.25rem; font-size: 1rem;",
                        value: "{full_name}",
                        oninput: move |e| full_name.set(e.value())
                    }
                }

                div {
                    style: "margin-bottom: 1rem;",
                    label { style: "display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem; color: #4a5568;", "Barcode" }
                    input {
                        r#type: "text",
                        style: "width: 100%; padding: 0.625rem; border: 1px solid #e2e8f0; border-radius: 0.25rem; font-size: 1rem;",
                        value: "{barcode}",
                        oninput: move |e| barcode.set(e.value())
                    }
                }

                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;",
                    div {
                        label { style: "display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem; color: #4a5568;", "Price *" }
                        input {
                            r#type: "number",
                            step: "0.01",
                            style: "width: 100%; padding: 0.625rem; border: 1px solid #e2e8f0; border-radius: 0.25rem; font-size: 1rem;",
                            value: "{price}",
                            oninput: move |e| price.set(e.value())
                        }
                    }
                    div {
                        label { style: "display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem; color: #4a5568;", "Cost" }
                        input {
                            r#type: "number",
                            step: "0.01",
                            style: "width: 100%; padding: 0.625rem; border: 1px solid #e2e8f0; border-radius: 0.25rem; font-size: 1rem;",
                            value: "{cost}",
                            oninput: move |e| cost.set(e.value())
                        }
                    }
                }

                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1.5rem;",
                    div {
                        label { style: "display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem; color: #4a5568;", "Stock Amount" }
                        input {
                            r#type: "number",
                            style: "width: 100%; padding: 0.625rem; border: 1px solid #e2e8f0; border-radius: 0.25rem; font-size: 1rem;",
                            value: "{stock}",
                            oninput: move |e| stock.set(e.value())
                        }
                    }
                    div {
                        label { style: "display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem; color: #4a5568;", "Min Stock" }
                        input {
                            r#type: "number",
                            style: "width: 100%; padding: 0.625rem; border: 1px solid #e2e8f0; border-radius: 0.25rem; font-size: 1rem;",
                            value: "{min_stock}",
                            oninput: move |e| min_stock.set(e.value())
                        }
                    }
                }

                div {
                    style: "margin-bottom: 1.5rem;",
                    label { style: "display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem; color: #4a5568;", "Unit of Measurement" }
                    select {
                        style: "width: 100%; padding: 0.625rem; border: 1px solid #e2e8f0; border-radius: 0.25rem; background: white; font-size: 1rem;",
                        onchange: move |evt| {
                            if let Ok(id) = evt.value().parse::<i32>() {
                                unit_id.set(id);
                            }
                        },
                        if let Some(Ok(units)) = units_resource.read().as_ref() {
                            for unit in units {
                                option {
                                    value: "{unit.id}",
                                    selected: unit.id == unit_id(),
                                    "{unit.description} ({unit.abbreviation})"
                                }
                            }
                        }
                    }
                }

                div {
                    style: "display: flex; justify-content: flex-end; gap: 0.5rem;",
                    button {
                        style: "padding: 0.5rem 1rem; border: 1px solid #e2e8f0; background: white; border-radius: 0.25rem; cursor: pointer; font-size: 1rem; font-weight: 500;",
                        onclick: move |_| on_close.call(()),
                        "Cancel"
                    }
                    button {
                        style: "padding: 0.5rem 1rem; background: #667eea; color: white; border: none; border-radius: 0.25rem; cursor: pointer; font-size: 1rem; font-weight: 500;",
                        onclick: handle_submit,
                        "{save_label}"
                    }
                }
            }
        }
    }
}
