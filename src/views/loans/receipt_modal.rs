//! Receipt Modal Component
//!
//! Modal dialog for displaying sale receipts linked to loans.

use crate::models::{Operation, Sale};
use crate::utils::formatting::format_currency;
use dioxus::prelude::*;

#[component]
pub fn ReceiptModal(sale: Sale, operations: Vec<Operation>, on_close: EventHandler<()>) -> Element {
    let formatted_date = sale.sold_at.format("%Y-%m-%d %H:%M:%S").to_string();

    rsx! {
        // Modal overlay
        div {
            style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;",
            onclick: move |_| on_close.call(()),

            // Modal content
            div {
                style: "background: white; padding: 2rem; border-radius: 0.5rem; max-width: 600px; width: 90%; max-height: 80vh; overflow-y: auto;",
                onclick: move |evt| evt.stop_propagation(),

                // Header
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem; padding-bottom: 1rem; border-bottom: 2px solid #e2e8f0;",
                    h3 {
                        style: "margin: 0; font-size: 1.5rem; font-weight: 600; color: #2d3748;",
                        "🧾 Sale Receipt"
                    }
                    button {
                        style: "background: transparent; border: none; font-size: 1.5rem; cursor: pointer; color: #718096;",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }

                // Sale info
                div {
                    style: "background: #f7fafc; padding: 1rem; border-radius: 0.5rem; margin-bottom: 1.5rem;",
                    div {
                        style: "margin-bottom: 0.5rem;",
                        span { style: "font-weight: 500; color: #4a5568;", "Sale ID: " }
                        span { style: "font-family: monospace; font-size: 0.875rem;", "{sale.id}" }
                    }
                    div {
                        style: "margin-bottom: 0.5rem;",
                        span { style: "font-weight: 500; color: #4a5568;", "Date: " }
                        span { "{formatted_date}" }
                    }
                    div {
                        span { style: "font-weight: 500; color: #4a5568;", "Type: " }
                        if sale.is_loan {
                            span {
                                style: "background: #fed7d7; color: #c53030; padding: 0.25rem 0.5rem; border-radius: 0.25rem; font-size: 0.875rem; font-weight: 500;",
                                "💳 Loan"
                            }
                        } else {
                            span {
                                style: "background: #c6f6d5; color: #22543d; padding: 0.25rem 0.5rem; border-radius: 0.25rem; font-size: 0.875rem; font-weight: 500;",
                                "💰 Cash"
                            }
                        }
                    }
                }

                // Items table
                div {
                    style: "margin-bottom: 1.5rem;",
                    h4 {
                        style: "margin: 0 0 1rem 0; font-size: 1rem; font-weight: 600; color: #2d3748;",
                        "Items"
                    }
                    table {
                        style: "width: 100%; border-collapse: collapse;",
                        thead {
                            tr {
                                style: "background: #f7fafc; border-bottom: 2px solid #e2e8f0;",
                                th { style: "padding: 0.75rem; text-align: left; font-weight: 600; color: #4a5568; font-size: 0.875rem;", "Product" }
                                th { style: "padding: 0.75rem; text-align: right; font-weight: 600; color: #4a5568; font-size: 0.875rem;", "Qty" }
                                th { style: "padding: 0.75rem; text-align: right; font-weight: 600; color: #4a5568; font-size: 0.875rem;", "Price" }
                                th { style: "padding: 0.75rem; text-align: right; font-weight: 600; color: #4a5568; font-size: 0.875rem;", "Subtotal" }
                            }
                        }
                        tbody {
                            for operation in &operations {
                                tr {
                                    style: "border-bottom: 1px solid #e2e8f0;",
                                    td { style: "padding: 0.75rem;", "{operation.product_name}" }
                                    td { style: "padding: 0.75rem; text-align: right; font-family: monospace;", "{operation.quantity:.3}" }
                                    td { style: "padding: 0.75rem; text-align: right; font-family: monospace;", "{format_currency(operation.unit_price)}" }
                                    td { style: "padding: 0.75rem; text-align: right; font-weight: 500; font-family: monospace;", "{format_currency(operation.subtotal)}" }
                                }
                            }
                        }
                    }
                }

                // Totals
                div {
                    style: "border-top: 2px solid #e2e8f0; padding-top: 1rem;",
                    div {
                        style: "display: flex; justify-content: space-between; margin-bottom: 0.5rem; font-size: 1.125rem;",
                        span { style: "font-weight: 500; color: #4a5568;", "Total:" }
                        span { style: "font-weight: 700; color: #2d3748; font-family: monospace;", "{format_currency(sale.total_amount)}" }
                    }
                    div {
                        style: "display: flex; justify-content: space-between; margin-bottom: 0.5rem;",
                        span { style: "font-weight: 500; color: #4a5568;", "Paid:" }
                        span { style: "color: #48bb78; font-weight: 600; font-family: monospace;", "{format_currency(sale.paid_amount)}" }
                    }
                    if sale.change_amount > rust_decimal::Decimal::ZERO {
                        div {
                            style: "display: flex; justify-content: space-between; margin-bottom: 0.5rem;",
                            span { style: "font-weight: 500; color: #4a5568;", "Change:" }
                            span { style: "color: #667eea; font-weight: 600; font-family: monospace;", "{format_currency(sale.change_amount)}" }
                        }
                    }
                }

                // Close button
                div {
                    style: "margin-top: 1.5rem;",
                    button {
                        style: "width: 100%; background: #667eea; color: white; padding: 0.75rem; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer;",
                        onclick: move |_| on_close.call(()),
                        "Close"
                    }
                }
            }
        }
    }
}
