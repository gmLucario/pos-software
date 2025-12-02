//! Payment History Modal Component
//!
//! Modal dialog for displaying loan payment history.

use crate::models::LoanPayment;
use crate::utils::formatting::format_currency;
use chrono_tz::America::Mexico_City;
use dioxus::prelude::*;

#[component]
pub fn PaymentHistoryModal(
    debtor_name: String,
    payments: Vec<LoanPayment>,
    on_close: EventHandler<()>,
) -> Element {
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
                        "💰 Payment History"
                    }
                    button {
                        style: "background: transparent; border: none; font-size: 1.5rem; cursor: pointer; color: #718096;",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }

                // Debtor info
                div {
                    style: "background: #f7fafc; padding: 1rem; border-radius: 0.5rem; margin-bottom: 1.5rem;",
                    div {
                        style: "font-weight: 600; color: #2d3748;",
                        "{debtor_name}"
                    }
                }

                if payments.is_empty() {
                    div {
                        style: "text-align: center; padding: 3rem; color: #a0aec0;",
                        p { "No payments recorded yet" }
                        p { style: "font-size: 3rem;", "💸" }
                    }
                } else {
                    // Payments table
                    div {
                        style: "margin-bottom: 1.5rem;",
                        h4 {
                            style: "margin: 0 0 1rem 0; font-size: 1rem; font-weight: 600; color: #2d3748;",
                            "Payment History ({payments.len()} payments)"
                        }
                        table {
                            style: "width: 100%; border-collapse: collapse;",
                            thead {
                                tr {
                                    style: "background: #f7fafc; border-bottom: 2px solid #e2e8f0;",
                                    th { style: "padding: 0.75rem; text-align: left; font-weight: 600; color: #4a5568; font-size: 0.875rem;", "Date" }
                                    th { style: "padding: 0.75rem; text-align: right; font-weight: 600; color: #4a5568; font-size: 0.875rem;", "Amount" }
                                    th { style: "padding: 0.75rem; text-align: left; font-weight: 600; color: #4a5568; font-size: 0.875rem;", "Notes" }
                                }
                            }
                            tbody {
                                for payment in &payments {
                                    {
                                        let formatted_date = payment.payment_date
                                            .with_timezone(&Mexico_City)
                                            .format("%d-%b-%Y %H:%M")
                                            .to_string();
                                        rsx! {
                                            tr {
                                                style: "border-bottom: 1px solid #e2e8f0;",
                                                td {
                                                    style: "padding: 0.75rem; font-family: monospace; font-size: 0.875rem;",
                                                    "{formatted_date}"
                                                }
                                                td {
                                                    style: "padding: 0.75rem; text-align: right; font-weight: 600; color: #48bb78; font-family: monospace;",
                                                    "{format_currency(payment.amount)}"
                                                }
                                                td {
                                                    style: "padding: 0.75rem; color: #718096; font-size: 0.875rem;",
                                                    "{payment.notes.as_deref().unwrap_or(\"-\")}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Total
                        div {
                            style: "border-top: 2px solid #e2e8f0; padding-top: 1rem; margin-top: 1rem;",
                            div {
                                style: "display: flex; justify-content: space-between; font-size: 1.125rem;",
                                span { style: "font-weight: 600; color: #2d3748;", "Total Paid:" }
                                span {
                                    style: "font-weight: 700; color: #48bb78; font-family: monospace;",
                                    {
                                        let total: rust_decimal::Decimal = payments.iter().map(|p| p.amount).sum();
                                        format_currency(total)
                                    }
                                }
                            }
                        }
                    }
                }

                // Close button
                div {
                    style: "margin-top: 1.5rem;",
                    button {
                        style: "width: 100%; background: #667eea; color: white; padding: 0.75rem; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer; font-size: 1rem;",
                        onclick: move |_| on_close.call(()),
                        "Close"
                    }
                }
            }
        }
    }
}
