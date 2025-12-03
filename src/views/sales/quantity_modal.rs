//! Quantity Modal Component
//!
//! Modal dialog for entering product quantity when adding to cart.

use crate::models::Product;
use crate::utils::formatting::format_currency;
use dioxus::prelude::*;

use super::validations::is_valid_quantity;

#[component]
pub fn QuantityModal(
    product: Product,
    unit_abbreviation: String,
    on_confirm: EventHandler<(Product, f64)>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut quantity_input = use_signal(|| String::from("1"));
    let mut has_invalid_input = use_signal(|| false);

    // Clone product for closures
    let product_for_keydown = product.clone();
    let product_for_button = product.clone();

    rsx! {
        // Modal overlay
        div {
            style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;",
            onclick: move |_| on_cancel.call(()),

            // Modal content
            div {
                style: "background: white; padding: 2rem; border-radius: 0.5rem; max-width: 400px; width: 90%;",
                onclick: move |evt| evt.stop_propagation(),

                h3 {
                    style: "margin: 0 0 1rem 0; color: #2d3748; font-size: 1.25rem;",
                    "Add to Cart"
                }

                // Product info
                div {
                    style: "background: #f7fafc; padding: 1rem; border-radius: 0.5rem; margin-bottom: 1.5rem;",
                    div {
                        style: "font-weight: 600; color: #2d3748; margin-bottom: 0.5rem; font-size: 1rem;",
                        "{product.full_name}"
                    }
                    div {
                        style: "color: #667eea; font-size: 1rem; font-weight: 700;",
                        "{format_currency(product.user_price)} / {unit_abbreviation}"
                    }
                    div {
                        style: "color: #718096; font-size: 0.875rem; margin-top: 0.5rem;",
                        "Available: {product.current_amount:.3} {unit_abbreviation}"
                    }
                }

                // Quantity input
                div {
                    style: "margin-bottom: 1.5rem;",
                    label {
                        style: "display: block; font-size: 0.875rem; font-weight: 500; color: #4a5568; margin-bottom: 0.5rem;",
                        "Quantity ({unit_abbreviation}):"
                    }
                    input {
                        r#type: "text",
                        inputmode: "decimal",
                        placeholder: "1.000",
                        value: "{quantity_input}",
                        autofocus: true,
                        oninput: move |evt| {
                            let value = evt.value();
                            // Allow partial input for better UX
                            if value.is_empty() || value == "." || value.ends_with('.') || is_valid_quantity(&value) {
                                has_invalid_input.set(false);
                                quantity_input.set(value);
                            } else {
                                has_invalid_input.set(true);
                            }
                        },
                        onkeydown: move |evt| {
                            if evt.key() == Key::Enter {
                                let quantity_str = quantity_input.read();
                                if let Ok(quantity) = quantity_str.parse::<f64>() {
                                    if is_valid_quantity(&quantity_str) {
                                        on_confirm.call((product_for_keydown.clone(), quantity));
                                    } else {
                                        has_invalid_input.set(true);
                                    }
                                }
                            } else if evt.key() == Key::Escape {
                                on_cancel.call(());
                            }
                        },
                        style: if *has_invalid_input.read() {
                            "width: 100%; padding: 0.75rem; border: 2px solid #f56565; border-radius: 0.5rem; font-size: 1rem; box-sizing: border-box; background: #fff5f5;"
                        } else {
                            "width: 100%; padding: 0.75rem; border: 2px solid #e2e8f0; border-radius: 0.5rem; font-size: 1rem; box-sizing: border-box;"
                        },
                    }
                    if *has_invalid_input.read() {
                        div {
                            style: "margin-top: 0.5rem; font-size: 0.875rem; color: #c53030;",
                            "⚠️ Enter a valid quantity (max 3 decimals, not exceeding stock)"
                        }
                    }
                }

                // Action buttons
                div {
                    style: "display: flex; gap: 0.75rem;",
                    button {
                        style: "flex: 1; background: #e2e8f0; color: #2d3748; padding: 0.75rem; border: none; border-radius: 0.5rem; cursor: pointer; font-weight: 500; font-size: 1rem;",
                        onclick: move |_| on_cancel.call(()),
                        "Cancel"
                    }
                    button {
                        style: "flex: 1; background: #48bb78; color: white; padding: 0.75rem; border: none; border-radius: 0.5rem; cursor: pointer; font-weight: 600; font-size: 1rem;",
                        onclick: move |_| {
                            let quantity_str = quantity_input.read();
                            if let Ok(quantity) = quantity_str.parse::<f64>() {
                                if is_valid_quantity(&quantity_str) {
                                    on_confirm.call((product_for_button.clone(), quantity));
                                } else {
                                    has_invalid_input.set(true);
                                }
                            }
                        },
                        "Add to Cart"
                    }
                }
            }
        }
    }
}
