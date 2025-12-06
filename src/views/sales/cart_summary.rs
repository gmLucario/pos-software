//! Cart Summary Component
//!
//! Displays cart total, payment input, change calculation, and checkout button.

use crate::utils::formatting::format_currency;
use dioxus::prelude::*;
use rust_decimal::Decimal;

use super::validations::is_valid_payment_amount;

#[component]
pub fn CartSummary(
    cart_total: Decimal,
    change_amount: ReadSignal<Decimal>,
    payment_amount: ReadSignal<String>,
    cart_is_empty: bool,
    on_payment_change: EventHandler<String>,
    on_complete_sale: EventHandler<()>,
) -> Element {
    let mut has_invalid_input = use_signal(|| false);

    // Read the change amount once for reactivity
    let current_change = change_amount();

    rsx! {
        div {
            style: "border-top: 2px solid #e2e8f0; padding-top: 1rem;",

            // Total
            div {
                style: "display: flex; justify-content: space-between; margin-bottom: 1rem; font-size: 1.5rem;",
                span { style: "font-weight: 700;", "Total:" }
                span { style: "font-weight: 700; color: #667eea;", "{format_currency(cart_total)}" }
            }

            // Payment input
            div {
                style: "margin-bottom: 1rem;",
                label {
                    style: "display: block; font-size: 0.875rem; font-weight: 500; color: #4a5568; margin-bottom: 0.5rem;",
                    "Payment Amount:"
                }
                input {
                    r#type: "text",
                    inputmode: "decimal",
                    placeholder: "0.00",
                    value: "{payment_amount}",
                    oninput: move |evt| {
                        let new_value = evt.value();
                        // Check if valid payment amount
                        if is_valid_payment_amount(&new_value) {
                            has_invalid_input.set(false);
                            on_payment_change.call(new_value);
                        } else {
                            // Show error state briefly
                            has_invalid_input.set(true);
                            // Don't update the payment amount in state
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
                        "⚠️ Only positive numbers with max 2 decimals allowed"
                    }
                }
            }

            // Change
            div {
                style: "display: flex; justify-content: space-between; margin-bottom: 0.5rem; font-size: 1rem;",
                span { style: "font-weight: 500;", "Change:" }
                span {
                    style: if current_change > Decimal::ZERO {
                        "font-weight: 600; color: #48bb78;"
                    } else {
                        "font-weight: 600; color: #718096;"
                    },
                    "{format_currency(current_change)}"
                }
            }

            // Complete sale button
            button {
                style: "width: 100%; background: #48bb78; color: white; padding: 1rem; border: none; border-radius: 0.5rem; font-size: 1rem; font-weight: 600; cursor: pointer; transition: background 0.2s;",
                disabled: cart_is_empty,
                onclick: move |_| on_complete_sale.call(()),
                "💳 Complete Sale"
            }
        }
    }
}
