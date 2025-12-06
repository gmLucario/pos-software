//! Loan PDF Receipt Modal Component
//!
//! Modal dialog for generating and saving loan receipts with payment history as PDF.

use crate::models::{Loan, LoanPayment, Operation, Sale};
use crate::utils::formatting::format_currency;
use chrono_tz::America::Mexico_City;
use dioxus::prelude::*;

#[component]
pub fn LoanPdfReceiptModal(
    loan: Loan,
    sale: Sale,
    operations: Vec<Operation>,
    payments: Vec<LoanPayment>,
    on_close: EventHandler<()>,
) -> Element {
    let formatted_date = sale
        .sold_at
        .with_timezone(&Mexico_City)
        .format("%d-%b-%Y %H:%M")
        .to_string();

    // Clone values for print handler
    let loan_clone = loan.clone();
    let sale_clone = sale.clone();
    let operations_clone = operations.clone();
    let payments_clone = payments.clone();

    // Print handler
    let print_receipt = move |_| {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let sale_id = sale_clone.id.clone();
            let loan_ref = &loan_clone;
            let sale_ref = &sale_clone;
            let ops_ref = &operations_clone;
            let payments_ref = &payments_clone;

            // Show file save dialog
            if let Some(file_path) = rfd::FileDialog::new()
                .set_file_name(format!("loan_receipt_{}.pdf", sale_id))
                .add_filter("PDF", &["pdf"])
                .save_file()
            {
                if let Err(e) = super::receipt_template::generate_loan_receipt_pdf(
                    loan_ref,
                    sale_ref,
                    ops_ref,
                    payments_ref,
                    file_path,
                ) {
                    tracing::error!("Failed to generate loan receipt: {}", e);
                }
            }
        }
    };

    let total_paid: rust_decimal::Decimal = payments.iter().map(|p| p.amount).sum();

    rsx! {
        // Modal overlay
        div {
            style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;",
            onclick: move |_| on_close.call(()),

            // Modal content
            div {
                style: "background: white; padding: 2rem; border-radius: 0.5rem; max-width: 700px; width: 90%; max-height: 80vh; overflow-y: auto;",
                onclick: move |evt| evt.stop_propagation(),

                // Header
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem; padding-bottom: 1rem; border-bottom: 2px solid #e2e8f0;",
                    h3 {
                        style: "margin: 0; font-size: 1.5rem; font-weight: 600; color: #2d3748;",
                        "📄 Loan Receipt"
                    }
                    button {
                        style: "background: transparent; border: none; font-size: 1.5rem; cursor: pointer; color: #718096;",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }

                // Debtor info
                div {
                    style: "background: #fffaf0; padding: 1rem; border-radius: 0.5rem; margin-bottom: 1.5rem; border: 1px solid #ed8936;",
                    div {
                        style: "font-weight: 600; color: #2d3748; margin-bottom: 0.5rem;",
                        "Customer: {loan.debtor_name}"
                    }
                    if let Some(phone) = &loan.debtor_phone {
                        div {
                            style: "color: #4a5568;",
                            "Phone: {phone}"
                        }
                    }
                }

                // Sale info
                div {
                    style: "background: #f7fafc; padding: 1rem; border-radius: 0.5rem; margin-bottom: 1.5rem;",
                    div {
                        style: "margin-bottom: 0.5rem;",
                        span { style: "font-weight: 500; color: #4a5568;", "Receipt #: " }
                        span { style: "font-family: monospace; font-size: 0.875rem;", "{sale.id}" }
                    }
                    div {
                        span { style: "font-weight: 500; color: #4a5568;", "Date: " }
                        span { "{formatted_date}" }
                    }
                }

                // Items table
                div {
                    style: "margin-bottom: 1.5rem;",
                    h4 {
                        style: "margin: 0 0 1rem 0; font-size: 1rem; font-weight: 600; color: #2d3748;",
                        "Items ({operations.len()})"
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

                // Total
                div {
                    style: "border-top: 2px solid #e2e8f0; padding-top: 1rem; margin-bottom: 1.5rem;",
                    div {
                        style: "display: flex; justify-content: space-between; font-size: 1.125rem;",
                        span { style: "font-weight: 600; color: #2d3748;", "Total:" }
                        span { style: "font-weight: 700; color: #2d3748; font-family: monospace;", "{format_currency(sale.total_amount)}" }
                    }
                }

                // Payment History
                div {
                    style: "margin-bottom: 1.5rem;",
                    h4 {
                        style: "margin: 0 0 1rem 0; font-size: 1rem; font-weight: 600; color: #2d3748;",
                        "Payment History ({payments.len()} payments)"
                    }
                    if payments.is_empty() {
                        div {
                            style: "text-align: center; padding: 2rem; color: #a0aec0; background: #f7fafc; border-radius: 0.5rem;",
                            "No payments recorded yet"
                        }
                    } else {
                        table {
                            style: "width: 100%; border-collapse: collapse;",
                            thead {
                                tr {
                                    style: "background: #f7fafc; border-bottom: 2px solid #e2e8f0;",
                                    th { style: "padding: 0.75rem; text-align: left; font-weight: 600; color: #4a5568; font-size: 0.875rem;", "Date" }
                                    th { style: "padding: 0.75rem; text-align: right; font-weight: 600; color: #4a5568; font-size: 0.875rem;", "Amount" }
                                }
                            }
                            tbody {
                                for payment in &payments {
                                    {
                                        let payment_date = payment.payment_date
                                            .with_timezone(&Mexico_City)
                                            .format("%d-%b-%Y %H:%M")
                                            .to_string();
                                        rsx! {
                                            tr {
                                                style: "border-bottom: 1px solid #e2e8f0;",
                                                td {
                                                    style: "padding: 0.75rem; font-family: monospace; font-size: 0.875rem;",
                                                    "{payment_date}"
                                                }
                                                td {
                                                    style: "padding: 0.75rem; text-align: right; font-weight: 600; color: #48bb78; font-family: monospace;",
                                                    "{format_currency(payment.amount)}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Total Paid
                        div {
                            style: "border-top: 2px solid #e2e8f0; padding-top: 1rem; margin-top: 1rem;",
                            div {
                                style: "display: flex; justify-content: space-between; font-size: 1.125rem;",
                                span { style: "font-weight: 600; color: #2d3748;", "Total Paid:" }
                                span {
                                    style: "font-weight: 700; color: #48bb78; font-family: monospace;",
                                    "{format_currency(total_paid)}"
                                }
                            }
                        }
                    }
                }

                // Payment Status
                div {
                    style: "border-top: 2px solid #e2e8f0; padding-top: 1rem; margin-bottom: 1.5rem;",
                    if loan.is_paid_off() {
                        div {
                            style: "text-align: center; padding: 1rem; background: #f0fff4; color: #22543d; border-radius: 0.5rem; border: 1px solid #48bb78;",
                            div {
                                style: "font-size: 2rem; margin-bottom: 0.5rem;",
                                "✓"
                            }
                            div {
                                style: "font-weight: 700; font-size: 1.125rem;",
                                "FULLY PAID"
                            }
                        }
                    } else {
                        div {
                            style: "display: flex; justify-content: space-between; font-size: 1.125rem;",
                            span { style: "font-weight: 600; color: #c53030;", "Amount Still Owed:" }
                            span { style: "font-weight: 700; color: #c53030; font-family: monospace;", "{format_currency(loan.remaining_amount)}" }
                        }
                    }
                }

                // Action buttons
                div {
                    style: "display: flex; gap: 1rem;",
                    button {
                        style: "flex: 1; background: #48bb78; color: white; padding: 0.75rem; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer; font-size: 1rem;",
                        onclick: print_receipt,
                        "💾 Save PDF"
                    }
                    button {
                        style: "flex: 1; background: #667eea; color: white; padding: 0.75rem; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer; font-size: 1rem;",
                        onclick: move |_| on_close.call(()),
                        "Close"
                    }
                }
            }
        }
    }
}
