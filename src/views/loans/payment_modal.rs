//! Payment Modal Component
//!
//! Modal dialog for recording loan payments.

use crate::models::Loan;
use crate::utils::formatting::format_currency;
use dioxus::prelude::*;

#[component]
pub fn PaymentModal(
    loan: Loan,
    payment_amount: String,
    payment_notes: String,
    on_amount_change: EventHandler<String>,
    on_notes_change: EventHandler<String>,
    on_cancel: EventHandler<()>,
    on_confirm: EventHandler<()>,
) -> Element {
    rsx! {
        // Modal overlay
        div {
            style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;",
            onclick: move |_| on_cancel.call(()),

            // Modal content
            div {
                style: "background: white; padding: 2rem; border-radius: 0.5rem; max-width: 500px; width: 100%;",
                onclick: move |evt| evt.stop_propagation(),

                h3 {
                    style: "margin: 0 0 1.5rem 0; font-size: 1.25rem; font-weight: 600; color: #2d3748;",
                    "💳 Record Payment"
                }

                // Loan details
                div {
                    style: "background: #f7fafc; padding: 1rem; border-radius: 0.5rem; margin-bottom: 1.5rem;",

                    div {
                        style: "margin-bottom: 0.5rem;",
                        span { style: "font-weight: 500;", "Debtor: " }
                        span { "{loan.debtor_name}" }
                    }
                    div {
                        style: "margin-bottom: 0.5rem;",
                        span { style: "font-weight: 500;", "Phone: " }
                        span { "{loan.debtor_phone.as_deref().unwrap_or(\"-\")}" }
                    }
                    div {
                        style: "margin-bottom: 0.5rem;",
                        span { style: "font-weight: 500;", "Remaining: " }
                        span { style: "color: #f56565; font-weight: 600;", "{format_currency(loan.remaining_amount)}" }
                    }
                }

                // Payment input
                div {
                    style: "margin-bottom: 1rem;",
                    label {
                        style: "display: block; font-size: 0.875rem; font-weight: 500; color: #4a5568; margin-bottom: 0.5rem;",
                        "Payment Amount"
                    }
                    input {
                        r#type: "number",
                        step: "0.01",
                        placeholder: "0.00",
                        value: "{payment_amount}",
                        autofocus: true,
                        oninput: move |evt| on_amount_change.call(evt.value()),
                        style: "width: 100%; padding: 0.75rem; border: 2px solid #e2e8f0; border-radius: 0.5rem; font-size: 1.125rem; box-sizing: border-box;",
                    }
                }

                // Notes input
                div {
                    style: "margin-bottom: 1.5rem;",
                    label {
                        style: "display: block; font-size: 0.875rem; font-weight: 500; color: #4a5568; margin-bottom: 0.5rem;",
                        "Notes (optional)"
                    }
                    textarea {
                        placeholder: "Add notes about this payment...",
                        value: "{payment_notes}",
                        oninput: move |evt| on_notes_change.call(evt.value()),
                        onkeydown: move |evt| {
                            if evt.key() == Key::Enter && evt.modifiers().ctrl() {
                                on_confirm.call(());
                            } else if evt.key() == Key::Escape {
                                on_cancel.call(());
                            }
                        },
                        style: "width: 100%; padding: 0.75rem; border: 2px solid #e2e8f0; border-radius: 0.5rem; font-size: 1rem; box-sizing: border-box; min-height: 80px; resize: vertical; font-family: inherit;",
                    }
                    div {
                        style: "font-size: 0.75rem; color: #718096; margin-top: 0.25rem;",
                        "Press Ctrl+Enter to submit"
                    }
                }

                // Buttons
                div {
                    style: "display: flex; gap: 1rem;",

                    button {
                        style: "flex: 1; background: #e2e8f0; color: #2d3748; padding: 0.75rem; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer;",
                        onclick: move |_| on_cancel.call(()),
                        "Cancel"
                    }

                    button {
                        style: "flex: 1; background: #48bb78; color: white; padding: 0.75rem; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer;",
                        onclick: move |_| on_confirm.call(()),
                        "💰 Record Payment"
                    }
                }
            }
        }
    }
}
