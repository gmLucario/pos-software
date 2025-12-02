//! Product Form Component
//!
//! Modal form for creating and editing products.

use crate::handlers::AppState;
use crate::models::{Product, ProductInput};
use dioxus::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;

#[component]
pub fn ProductForm(
    on_close: EventHandler<()>,
    on_save: EventHandler<ProductInput>,
    on_delete: EventHandler<String>,
    initial_product: Option<Product>,
) -> Element {
    // Clone initial_product to avoid lifetime issues
    let product_clone = initial_product.clone();

    let mut full_name = use_signal(|| extract_field(&product_clone, |p| p.full_name.clone()));
    let mut barcode = use_signal(|| extract_optional_field(&product_clone, |p| p.barcode.clone()));
    let mut price = use_signal(|| extract_field(&product_clone, |p| p.user_price.to_string()));
    let mut cost = use_signal(|| extract_optional_field(&product_clone, |p| p.cost_price.map(|c| c.to_string())));
    let mut stock = use_signal(|| extract_field(&product_clone, |p| p.current_amount.to_string()));
    let mut min_stock = use_signal(|| extract_field(&product_clone, |p| p.min_amount.to_string()));
    let mut unit_id = use_signal(|| product_clone.as_ref().map(|p| p.unit_measurement_id).unwrap_or(3));
    let mut error_msg = use_signal(String::new);

    let units_resource = use_resource(move || async move {
        let app_state = use_context::<AppState>();
        app_state.inventory_handler.get_units().await
    });

    let handle_submit = move |_| {
        match validate_and_build_product_input(
            &full_name(),
            &barcode(),
            &price(),
            &cost(),
            &stock(),
            &min_stock(),
            unit_id(),
        ) {
            Ok(input) => {
                error_msg.set(String::new());
                on_save.call(input);
            }
            Err(err) => {
                error_msg.set(err);
            }
        }
    };

    let (title, save_label) = get_form_labels(&initial_product);

    rsx! {
        div {
            style: "position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.5); display: flex; justify-content: center; align-items: center; z-index: 1000;",
            onclick: move |_| on_close.call(()),

            div {
                style: "background: white; padding: 2rem; border-radius: 0.5rem; width: 500px; max-width: 90%; max-height: 90vh; overflow-y: auto;",
                onclick: move |evt| evt.stop_propagation(),

                // Header
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;",
                    h3 { style: "margin: 0; font-size: 1.25rem;", "{title}" }

                    if let Some(product) = &initial_product {
                        button {
                            style: "background: #e53e3e; color: white; border: none; padding: 0.5rem 1rem; border-radius: 0.25rem; cursor: pointer; font-size: 0.875rem; font-weight: 500;",
                            onclick: move |_| on_delete.call(product.id.clone()),
                            "Delete Product"
                        }
                    }
                }

                // Error message
                if !error_msg().is_empty() {
                    div {
                        style: "background: #fff5f5; color: #c53030; padding: 0.75rem; border-radius: 0.25rem; margin-bottom: 1rem;",
                        "{error_msg}"
                    }
                }

                // Product Name
                { form_input("Product Name *", "text", full_name, None, None) }

                // Barcode
                { form_input("Barcode", "text", barcode, None, None) }

                // Price and Cost
                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;",
                    { form_input("Price *", "number", price, Some("0.01"), None) }
                    { form_input("Cost", "number", cost, Some("0.01"), None) }
                }

                // Stock Amount and Min Stock
                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1.5rem;",
                    { form_input("Stock Amount", "number", stock, None, None) }
                    { form_input("Min Stock", "number", min_stock, None, None) }
                }

                // Unit of Measurement
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

                // Action Buttons
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

/// Helper to extract a field from an optional product
fn extract_field<F, T>(product: &Option<Product>, extractor: F) -> T
where
    F: Fn(&Product) -> T,
    T: Default,
{
    product.as_ref().map(extractor).unwrap_or_default()
}

/// Helper to extract an optional field from an optional product
fn extract_optional_field<F, T>(product: &Option<Product>, extractor: F) -> T
where
    F: Fn(&Product) -> Option<T>,
    T: Default,
{
    product.as_ref().and_then(extractor).unwrap_or_default()
}

/// Helper to get form title and save button label
fn get_form_labels(initial_product: &Option<Product>) -> (&'static str, &'static str) {
    if initial_product.is_some() {
        ("Edit Product", "Update Product")
    } else {
        ("Add New Product", "Save Product")
    }
}

/// Validate form inputs and build ProductInput
fn validate_and_build_product_input(
    full_name: &str,
    barcode: &str,
    price: &str,
    cost: &str,
    stock: &str,
    min_stock: &str,
    unit_id: i32,
) -> Result<ProductInput, String> {
    // Validate product name
    if full_name.trim().is_empty() {
        return Err("Product name is required".to_string());
    }

    // Parse and validate price
    let user_price = Decimal::from_str(price)
        .map_err(|_| "Invalid price".to_string())?;

    // Parse optional cost
    let cost_price = if cost.is_empty() {
        None
    } else {
        Some(Decimal::from_str(cost).map_err(|_| "Invalid cost price".to_string())?)
    };

    // Parse stock amounts
    let current_amount = stock.parse::<f64>().unwrap_or(0.0);
    let min_amount = min_stock.parse::<f64>().unwrap_or(0.0);

    Ok(ProductInput {
        full_name: full_name.to_string(),
        barcode: if barcode.is_empty() {
            None
        } else {
            Some(barcode.to_string())
        },
        user_price,
        cost_price,
        current_amount,
        min_amount,
        unit_measurement_id: unit_id,
    })
}

/// Helper component for form input fields
fn form_input(
    label: &str,
    input_type: &str,
    mut signal: Signal<String>,
    step: Option<&str>,
    wrapper_style: Option<&str>,
) -> Element {
    rsx! {
        div {
            style: wrapper_style.unwrap_or("margin-bottom: 1rem;"),
            label {
                style: "display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem; color: #4a5568;",
                "{label}"
            }
            input {
                r#type: "{input_type}",
                step: step,
                style: "width: 100%; padding: 0.625rem; border: 1px solid #e2e8f0; border-radius: 0.25rem; font-size: 1rem;",
                value: "{signal}",
                oninput: move |e| signal.set(e.value())
            }
        }
    }
}
