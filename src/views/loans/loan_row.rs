//! Loan Row Component
//!
//! Displays a single loan row in the loans table.

use crate::models::Loan;
use crate::utils::formatting::format_currency;
use dioxus::prelude::*;

#[component]
pub fn LoanRow(
    loan: Loan,
    on_select: EventHandler<Loan>,
    on_view_receipt: EventHandler<String>,
    on_view_payment_history: EventHandler<String>,
    on_print_pdf: EventHandler<String>,
) -> Element {
    let is_paid = loan.is_paid_off();

    // Clone loan for closures
    let loan_for_receipt = loan.clone();
    let loan_for_payment = loan.clone();
    let loan_for_history = loan.clone();
    let loan_for_pdf = loan.clone();

    rsx! {
        tr {
            style: "border-bottom: 1px solid #e2e8f0;",

            td {
                style: "padding: 0.75rem; font-weight: 500;",
                "{loan.debtor_name}"
            }
            td {
                style: "padding: 0.75rem; color: #718096; font-family: monospace;",
                "{loan.debtor_phone.as_deref().unwrap_or(\"-\")}"
            }
            td {
                style: "padding: 0.75rem; text-align: center; font-weight: 500;",
                "{format_currency(loan.total_debt)}"
            }
            td {
                style: "padding: 0.75rem; text-align: center;",
                button {
                    style: "background: transparent; border: none; color: #48bb78; font-weight: 600; cursor: pointer; text-decoration: underline; font-family: inherit; font-size: inherit; padding: 0;",
                    onclick: move |_| on_view_payment_history.call(loan_for_history.id.clone()),
                    "{format_currency(loan.paid_amount)}"
                }
            }
            td {
                style: "padding: 0.75rem; text-align: center; color: #f56565; font-weight: 600;",
                "{format_currency(loan.remaining_amount)}"
            }
            td {
                style: "padding: 0.75rem; text-align: center;",
                button {
                    style: "background: #48bb78; color: white; padding: 0.5rem 1rem; border: none; border-radius: 0.25rem; cursor: pointer; font-size: 0.875rem; font-weight: 500; margin-right: 0.5rem;",
                    onclick: move |_| on_view_receipt.call(loan_for_receipt.id.clone()),
                    "🧾 Receipt"
                }
            }
            td {
                style: "padding: 0.75rem; text-align: center;",
                button {
                    style: "background: #ed8936; color: white; padding: 0.5rem 1rem; border: none; border-radius: 0.25rem; cursor: pointer; font-size: 0.875rem; font-weight: 500;",
                    onclick: move |_| on_print_pdf.call(loan_for_pdf.id.clone()),
                    "📄 Print PDF"
                }
            }
            td {
                style: "padding: 0.75rem; text-align: center;",
                if !is_paid {
                    button {
                        style: "background: #667eea; color: white; padding: 0.5rem 1rem; border: none; border-radius: 0.25rem; cursor: pointer; font-size: 0.875rem; font-weight: 500;",
                        onclick: move |_| on_select.call(loan_for_payment.clone()),
                        "💳 Pay"
                    }
                } else {
                    span {
                        style: "background: #f0fff4; color: #22543d; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.875rem; font-weight: 500;",
                        "✓ Paid"
                    }
                }
            }
        }
    }
}
