//! Loans Module
//!
//! UI components for tracking and managing customer loans.

use dioxus::prelude::*;
use rust_decimal::Decimal;

use crate::mock_data::MockLoan;

#[component]
pub fn LoansView(loans: Signal<Vec<MockLoan>>) -> Element {
    let mut search_query = use_signal(String::new);
    let mut selected_loan = use_signal(|| Option::<MockLoan>::None);
    let mut payment_amount = use_signal(String::new);

    // Filter loans based on search
    let filtered_loans = loans.read().iter()
        .filter(|loan| {
            let query = search_query.read().to_lowercase();
            if query.is_empty() {
                return true;
            }
            loan.debtor_name.to_lowercase().contains(&query) ||
            loan.debtor_phone.contains(&query)
        })
        .cloned()
        .collect::<Vec<_>>();

    // Calculate totals
    let total_debt: Decimal = loans.read().iter().map(|l| l.total_debt).sum();
    let total_paid: Decimal = loans.read().iter().map(|l| l.paid_amount).sum();
    let total_remaining: Decimal = loans.read().iter().map(|l| l.remaining).sum();

    rsx! {
        div {
            class: "loans-view",

            // Stats cards
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem; margin-bottom: 1.5rem;",

                StatCard {
                    label: "Total Debt",
                    value: format!("${}", total_debt),
                    color: "#f56565",
                    icon: "💳",
                }

                StatCard {
                    label: "Total Paid",
                    value: format!("${}", total_paid),
                    color: "#48bb78",
                    icon: "💰",
                }

                StatCard {
                    label: "Remaining",
                    value: format!("${}", total_remaining),
                    color: "#ed8936",
                    icon: "⏳",
                }

                StatCard {
                    label: "Active Loans",
                    value: format!("{}", loans.read().len()),
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
                                th { style: "padding: 0.75rem; text-align: left; font-weight: 600; color: #4a5568;", "Status" }
                                th { style: "padding: 0.75rem; text-align: center; font-weight: 600; color: #4a5568;", "Actions" }
                            }
                        }

                        tbody {
                            if filtered_loans.is_empty() {
                                tr {
                                    td {
                                        colspan: "8",
                                        style: "padding: 3rem; text-align: center; color: #a0aec0;",
                                        "No loans found"
                                    }
                                }
                            } else {
                                for loan in filtered_loans {
                                    LoanRow {
                                        loan: loan.clone(),
                                        on_select: move |l: MockLoan| selected_loan.set(Some(l)),
                                    }
                                }
                            }
                        }
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
                                span { "{loan.debtor_phone}" }
                            }
                            div {
                                style: "margin-bottom: 0.5rem;",
                                span { style: "font-weight: 500;", "Remaining: " }
                                span { style: "color: #f56565; font-weight: 600;", "${loan.remaining}" }
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
                                onclick: move |_| {
                                    // In real app, this would save the payment
                                    selected_loan.set(None);
                                    payment_amount.set(String::new());
                                },
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
fn LoanRow(loan: MockLoan, on_select: EventHandler<MockLoan>) -> Element {
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
                "{loan.debtor_phone}"
            }
            td {
                style: "padding: 0.75rem; text-align: right; font-weight: 500;",
                "${loan.total_debt}"
            }
            td {
                style: "padding: 0.75rem; text-align: right; color: #48bb78; font-weight: 500;",
                "${loan.paid_amount}"
            }
            td {
                style: "padding: 0.75rem; text-align: right; color: #f56565; font-weight: 600;",
                "${loan.remaining}"
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
                style: "padding: 0.75rem;",
                if is_paid {
                    span {
                        style: "background: #f0fff4; color: #22543d; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.875rem; font-weight: 500;",
                        "✓ Paid"
                    }
                } else {
                    span {
                        style: "background: #fffaf0; color: #c05621; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.875rem; font-weight: 500;",
                        "{loan.status}"
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
