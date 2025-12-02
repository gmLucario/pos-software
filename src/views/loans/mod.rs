//! Loans Module
//!
//! UI components for tracking and managing customer loans.

mod loan_form;
mod loan_row;
mod payment_history_modal;
mod payment_modal;
mod receipt_modal;
mod stat_card;

pub use loan_form::LoanForm;
use loan_row::LoanRow;
use payment_history_modal::PaymentHistoryModal;
use payment_modal::PaymentModal;
use receipt_modal::ReceiptModal;
use stat_card::StatCard;

use crate::handlers::AppState;
use crate::models::{Loan, LoanPayment, LoanPaymentInput, Operation, Sale};
use crate::utils::formatting::format_currency;
use dioxus::prelude::*;
use rust_decimal::Decimal;

#[component]
pub fn LoansView() -> Element {
    // Get app state from context
    let app_state = use_context::<AppState>();

    let mut search_query = use_signal(String::new);
    let mut selected_loan = use_signal(|| Option::<Loan>::None);
    let mut payment_amount = use_signal(String::new);
    let mut payment_notes = use_signal(String::new);
    let mut payment_message = use_signal(|| Option::<(bool, String)>::None);
    let mut refresh_trigger = use_signal(|| 0);
    let mut selected_receipt = use_signal(|| Option::<(Sale, Vec<Operation>)>::None);
    let mut selected_payment_history = use_signal(|| Option::<(String, Vec<LoanPayment>)>::None);

    // Load loans from database
    let mut loans_resource = use_resource({
        let loans_handler = app_state.loans_handler.clone();
        move || {
            let handler = loans_handler.clone();
            async move { handler.load_loans().await }
        }
    });

    // Refresh loans when trigger changes
    use_effect(move || {
        let _ = refresh_trigger();
        loans_resource.restart();
    });

    // Clone app_state for closures
    let app_state_for_payment = app_state.clone();
    let app_state_for_receipt = app_state.clone();
    let app_state_for_history = app_state.clone();

    // View receipt handler (as callback so it can be copied in the loop)
    let view_receipt_handler = use_callback(move |sale_id: String| {
        let app_state = app_state_for_receipt.clone();
        spawn(async move {
            match app_state.sales_handler.get_sale_details(sale_id).await {
                Ok(sale_with_ops) => {
                    selected_receipt.set(Some((sale_with_ops.sale, sale_with_ops.operations)));
                }
                Err(err) => {
                    payment_message.set(Some((false, format!("Failed to load receipt: {}", err))));
                }
            }
        });
    });

    // View payment history handler (as callback so it can be copied in the loop)
    let view_payment_history_handler = use_callback(move |loan_id: String| {
        let app_state = app_state_for_history.clone();
        spawn(async move {
            match app_state.loans_handler.get_loan_details(loan_id).await {
                Ok(loan_with_payments) => {
                    selected_payment_history.set(Some((
                        loan_with_payments.loan.debtor_name,
                        loan_with_payments.payments,
                    )));
                }
                Err(err) => {
                    payment_message.set(Some((
                        false,
                        format!("Failed to load payment history: {}", err),
                    )));
                }
            }
        });
    });

    // Record payment
    let record_payment = move |_| {
        let app_state = app_state_for_payment.clone();
        let loan_id = selected_loan.read().as_ref().map(|l| l.id.clone());
        let payment = payment_amount.read().clone();
        let notes = payment_notes.read().clone();

        spawn(async move {
            let Some(loan_id) = loan_id else {
                return;
            };

            // Parse payment amount
            let amount = match payment.parse::<Decimal>() {
                Ok(amount) if amount > Decimal::ZERO => amount,
                Ok(_) => {
                    payment_message.set(Some((
                        false,
                        "Payment amount must be greater than zero".to_string(),
                    )));
                    return;
                }
                Err(_) => {
                    payment_message.set(Some((false, "Invalid payment amount".to_string())));
                    return;
                }
            };

            // Create payment input with notes
            let payment_input = LoanPaymentInput {
                loan_id: loan_id.clone(),
                amount,
                notes: if notes.trim().is_empty() {
                    None
                } else {
                    Some(notes.trim().to_string())
                },
            };

            // Record the payment
            match app_state.loans_handler.record_payment(payment_input).await {
                Ok(_) => {
                    payment_message.set(Some((true, "Payment recorded successfully!".to_string())));
                    selected_loan.set(None);
                    payment_amount.set(String::new());
                    payment_notes.set(String::new());
                    refresh_trigger.set(refresh_trigger() + 1);
                }
                Err(err) => {
                    payment_message.set(Some((false, format!("Payment failed: {}", err))));
                }
            }
        });
    };

    rsx! {
        div {
            class: "loans-view",

            // Content based on loading state
            match &*loans_resource.read_unchecked() {
                Some(Ok(loans)) => {
                    // Filter loans based on search
                    let filtered_loans: Vec<Loan> = loans.iter()
                        .filter(|loan| {
                            let query = search_query.read().to_lowercase();
                            if query.is_empty() {
                                return true;
                            }
                            loan.debtor_name.to_lowercase().contains(&query) ||
                            loan.debtor_phone.as_ref().is_some_and(|p| p.contains(&query))
                        })
                        .cloned()
                        .collect();

                    // Calculate totals
                    let total_debt: Decimal = filtered_loans.iter().map(|l| l.total_debt).sum();
                    let total_paid: Decimal = filtered_loans.iter().map(|l| l.paid_amount).sum();
                    let total_remaining: Decimal = filtered_loans.iter().map(|l| l.remaining_amount).sum();

                    rsx! {
                        // Stats cards
                        div {
                            style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem; margin-bottom: 1.5rem;",

                            StatCard {
                                label: "Total Debt",
                                value: format_currency(total_debt),
                                color: "#f56565",
                                icon: "💳",
                            }

                            StatCard {
                                label: "Total Paid",
                                value: format_currency(total_paid),
                                color: "#48bb78",
                                icon: "💰",
                            }

                            StatCard {
                                label: "Remaining",
                                value: format_currency(total_remaining),
                                color: "#ed8936",
                                icon: "⏳",
                            }

                            StatCard {
                                label: "Active Loans",
                                value: format!("{}", filtered_loans.len()),
                                color: "#667eea",
                                icon: "📊",
                            }
                        }

                        // Main content
                        div {
                            style: "background: white; border-radius: 0.5rem; padding: 1.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",

                            // Header
                            div {
                                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;",

                                h2 {
                                    style: "font-size: 1.5rem; font-weight: 600; color: #2d3748; margin: 0;",
                                    "💰 Customer Loans"
                                }
                            }

                            // Payment message
                            if let Some((is_success, message)) = payment_message.read().clone() {
                                div {
                                    style: if is_success {
                                        "padding: 0.75rem; margin-bottom: 1rem; background: #f0fff4; color: #22543d; border-radius: 0.5rem; border: 1px solid #48bb78;"
                                    } else {
                                        "padding: 0.75rem; margin-bottom: 1rem; background: #fff5f5; color: #c53030; border-radius: 0.5rem; border: 1px solid #f56565;"
                                    },
                                    "{message}"
                                    button {
                                        style: "float: right; background: transparent; border: none; cursor: pointer; font-weight: bold;",
                                        onclick: move |_| payment_message.set(None),
                                        "✕"
                                    }
                                }
                            }

                            // Search bar
                            div {
                                style: "margin-bottom: 1.5rem;",

                                input {
                                    r#type: "text",
                                    placeholder: "🔍 Search by name or phone...",
                                    value: "{search_query}",
                                    oninput: move |evt| search_query.set(evt.value().clone()),
                                    style: "width: 100%; padding: 0.75rem; border: 2px solid #e2e8f0; border-radius: 0.5rem; font-size: 1rem; box-sizing: border-box;",
                                }
                            }

                            // Loans table
                            div {
                                style: "overflow-x: auto;",

                                table {
                                    style: "width: 100%; border-collapse: collapse;",

                                    thead {
                                        tr {
                                            style: "background: #f7fafc; border-bottom: 2px solid #e2e8f0;",

                                            th { style: "padding: 0.75rem; text-align: left; font-weight: 600; color: #4a5568;", "Debtor" }
                                            th { style: "padding: 0.75rem; text-align: left; font-weight: 600; color: #4a5568;", "Phone" }
                                            th { style: "padding: 0.75rem; text-align: center; font-weight: 600; color: #4a5568;", "Total Debt" }
                                            th { style: "padding: 0.75rem; text-align: center; font-weight: 600; color: #4a5568;", "Paid" }
                                            th { style: "padding: 0.75rem; text-align: center; font-weight: 600; color: #4a5568;", "Remaining" }
                                            th { style: "padding: 0.75rem; text-align: center; font-weight: 600; color: #4a5568;", "Receipt" }
                                            th { style: "padding: 0.75rem; text-align: center; font-weight: 600; color: #4a5568;", "Payment" }
                                        }
                                    }

                                    tbody {
                                        if filtered_loans.is_empty() {
                                            tr {
                                                td {
                                                    colspan: "7",
                                                    style: "padding: 3rem; text-align: center; color: #a0aec0;",
                                                    "No loans found"
                                                }
                                            }
                                        } else {
                                            for loan in filtered_loans {
                                                LoanRow {
                                                    loan: loan.clone(),
                                                    on_select: move |l: Loan| selected_loan.set(Some(l)),
                                                    on_view_receipt: view_receipt_handler,
                                                    on_view_payment_history: view_payment_history_handler,
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(err)) => rsx! {
                    div {
                        style: "padding: 2rem; text-align: center; color: #e53e3e; background: #fff5f5; border-radius: 0.5rem;",
                        "❌ Error loading loans: {err}"
                    }
                },
                None => rsx! {
                    div {
                        style: "padding: 2rem; text-align: center; color: #718096;",
                        "⏳ Loading loans..."
                    }
                }
            }

            // Payment modal
            if let Some(loan) = selected_loan.read().as_ref() {
                PaymentModal {
                    loan: loan.clone(),
                    payment_amount: payment_amount.read().clone(),
                    payment_notes: payment_notes.read().clone(),
                    on_amount_change: move |value: String| payment_amount.set(value),
                    on_notes_change: move |value: String| payment_notes.set(value),
                    on_cancel: move |_| {
                        selected_loan.set(None);
                        payment_amount.set(String::new());
                        payment_notes.set(String::new());
                    },
                    on_confirm: record_payment,
                }
            }

            // Receipt modal
            if let Some((sale, operations)) = selected_receipt.read().as_ref() {
                ReceiptModal {
                    sale: sale.clone(),
                    operations: operations.clone(),
                    on_close: move |_| selected_receipt.set(None),
                }
            }

            // Payment history modal
            if let Some((debtor_name, payments)) = selected_payment_history.read().as_ref() {
                PaymentHistoryModal {
                    debtor_name: debtor_name.clone(),
                    payments: payments.clone(),
                    on_close: move |_| selected_payment_history.set(None),
                }
            }
        }
    }
}
