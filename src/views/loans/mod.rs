//! Loans Module
//!
//! UI components for tracking and managing customer loans.

use dioxus::prelude::*;
use rust_decimal::Decimal;
use crate::handlers::AppState;
use crate::models::{Loan, LoanPaymentInput};
use crate::utils::formatting::format_currency;

#[component]
pub fn LoansView() -> Element {
    // Get app state from context
    let app_state = use_context::<AppState>();

    let mut search_query = use_signal(String::new);
    let mut selected_loan = use_signal(|| Option::<Loan>::None);
    let mut payment_amount = use_signal(String::new);
    let mut payment_message = use_signal(|| Option::<(bool, String)>::None);
    let mut refresh_trigger = use_signal(|| 0);

    // Load loans from database
    let loans_resource = use_resource(move || {
        let handler = app_state.loans_handler.clone();
        async move {
            handler.load_loans().await
        }
    });

    // Refresh loans when trigger changes
    use_effect(move || {
        let _ = refresh_trigger();
        loans_resource.restart();
    });

    // Record payment
    let record_payment = move |_| {
        let app_state = app_state.clone();
        let loan_id = selected_loan.read().as_ref().map(|l| l.id.clone());
        let payment = payment_amount.read().clone();

        spawn(async move {
            let Some(loan_id) = loan_id else {
                return;
            };

            // Parse payment amount
            let amount = match payment.parse::<Decimal>() {
                Ok(amount) if amount > Decimal::ZERO => amount,
                Ok(_) => {
                    payment_message.set(Some((false, "Payment amount must be greater than zero".to_string())));
                    return;
                }
                Err(_) => {
                    payment_message.set(Some((false, "Invalid payment amount".to_string())));
                    return;
                }
            };

            // Create payment input
            let payment_input = LoanPaymentInput {
                loan_id: loan_id.clone(),
                amount,
                notes: None,
            };

            // Record the payment
            match app_state.loans_handler.record_payment(payment_input).await {
                Ok(_) => {
                    payment_message.set(Some((true, "Payment recorded successfully!".to_string())));
                    selected_loan.set(None);
                    payment_amount.set(String::new());
                    refresh_trigger.set(refresh_trigger() + 1);
                },
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

                                button {
                                    style: "background: #48bb78; color: white; padding: 0.75rem 1.5rem; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer;",
                                    onclick: move |_| refresh_trigger.set(refresh_trigger() + 1),
                                    "🔄 Refresh"
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
                                            th { style: "padding: 0.75rem; text-align: right; font-weight: 600; color: #4a5568;", "Total Debt" }
                                            th { style: "padding: 0.75rem; text-align: right; font-weight: 600; color: #4a5568;", "Paid" }
                                            th { style: "padding: 0.75rem; text-align: right; font-weight: 600; color: #4a5568;", "Remaining" }
                                            th { style: "padding: 0.75rem; text-align: center; font-weight: 600; color: #4a5568;", "Progress" }
                                            th { style: "padding: 0.75rem; text-align: center; font-weight: 600; color: #4a5568;", "Actions" }
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
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;",
                    onclick: move |_| selected_loan.set(None),

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
                            style: "margin-bottom: 1.5rem;",
                            label {
                                style: "display: block; font-size: 0.875rem; font-weight: 500; color: #4a5568; margin-bottom: 0.5rem;",
                                "Payment Amount"
                            }
                            input {
                                r#type: "number",
                                step: "0.01",
                                placeholder: "0.00",
                                value: "{payment_amount}",
                                oninput: move |evt| payment_amount.set(evt.value().clone()),
                                style: "width: 100%; padding: 0.75rem; border: 2px solid #e2e8f0; border-radius: 0.5rem; font-size: 1.125rem; box-sizing: border-box;",
                            }
                        }

                        // Buttons
                        div {
                            style: "display: flex; gap: 1rem;",

                            button {
                                style: "flex: 1; background: #e2e8f0; color: #2d3748; padding: 0.75rem; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer;",
                                onclick: move |_| {
                                    selected_loan.set(None);
                                    payment_amount.set(String::new());
                                },
                                "Cancel"
                            }

                            button {
                                style: "flex: 1; background: #48bb78; color: white; padding: 0.75rem; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer;",
                                onclick: record_payment,
                                "💰 Record Payment"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LoanRow(loan: Loan, on_select: EventHandler<Loan>) -> Element {
    let percentage = loan.payment_percentage();
    let is_paid = loan.is_paid_off();

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
                style: "padding: 0.75rem; text-align: right; font-weight: 500;",
                "{format_currency(loan.total_debt)}"
            }
            td {
                style: "padding: 0.75rem; text-align: right; color: #48bb78; font-weight: 500;",
                "{format_currency(loan.paid_amount)}"
            }
            td {
                style: "padding: 0.75rem; text-align: right; color: #f56565; font-weight: 600;",
                "{format_currency(loan.remaining_amount)}"
            }
            td {
                style: "padding: 0.75rem;",
                div {
                    style: "width: 100px;",
                    div {
                        style: "background: #e2e8f0; height: 8px; border-radius: 9999px; overflow: hidden;",
                        div {
                            style: "background: #48bb78; height: 100%; width: {percentage}%; transition: width 0.3s;",
                        }
                    }
                    div {
                        style: "font-size: 0.75rem; color: #718096; margin-top: 0.25rem; text-align: center;",
                        "{percentage:.0}%"
                    }
                }
            }
            td {
                style: "padding: 0.75rem; text-align: center;",
                if !is_paid {
                    button {
                        style: "background: #667eea; color: white; padding: 0.5rem 1rem; border: none; border-radius: 0.25rem; cursor: pointer; font-size: 0.875rem; font-weight: 500;",
                        onclick: move |_| on_select.call(loan.clone()),
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

#[component]
fn StatCard(label: String, value: String, color: String, icon: String) -> Element {
    rsx! {
        div {
            style: "background: white; padding: 1.5rem; border-radius: 0.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1); border-left: 4px solid {color};",

            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;",
                span {
                    style: "font-size: 0.875rem; color: #718096; font-weight: 500;",
                    "{label}"
                }
                span {
                    style: "font-size: 1.5rem;",
                    "{icon}"
                }
            }
            div {
                style: "font-size: 1.75rem; font-weight: 700; color: {color};",
                "{value}"
            }
        }
    }
}
