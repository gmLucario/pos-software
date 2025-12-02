//! Loan Form Component
//!
//! Form for collecting debtor information when creating a loan.

use dioxus::prelude::*;

#[component]
pub fn LoanForm(
    debtor_name: String,
    debtor_phone: String,
    on_name_change: EventHandler<String>,
    on_phone_change: EventHandler<String>,
    on_cancel: EventHandler<()>,
    on_confirm: EventHandler<()>,
) -> Element {
    let is_valid = !debtor_name.trim().is_empty();

    rsx! {
        // Modal overlay
        div {
            style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;",
            onclick: move |_| on_cancel.call(()),

            // Modal content
            div {
                style: "background: white; padding: 2rem; border-radius: 0.5rem; max-width: 500px; width: 90%;",
                onclick: move |evt| evt.stop_propagation(),

                h3 {
                    style: "margin: 0 0 1rem 0; color: #2d3748; font-size: 1.25rem;",
                    "Loan Information"
                }

                div {
                    style: "background: #fffaf0; border: 1px solid #ed8936; padding: 1rem; border-radius: 0.5rem; margin-bottom: 1.5rem;",
                    div {
                        style: "color: #7c2d12; font-size: 0.875rem;",
                        "⚠️ This sale will be recorded as a loan. Please provide the debtor's information."
                    }
                }

                // Debtor name input
                div {
                    style: "margin-bottom: 1.5rem;",
                    label {
                        style: "display: block; font-size: 0.875rem; font-weight: 500; color: #4a5568; margin-bottom: 0.5rem;",
                        "Debtor Name *"
                    }
                    input {
                        r#type: "text",
                        placeholder: "Enter debtor name",
                        value: "{debtor_name}",
                        autofocus: true,
                        oninput: move |evt| on_name_change.call(evt.value()),
                        onkeydown: move |evt| {
                            if evt.key() == Key::Enter && is_valid {
                                on_confirm.call(());
                            } else if evt.key() == Key::Escape {
                                on_cancel.call(());
                            }
                        },
                        style: "width: 100%; padding: 0.75rem; border: 2px solid #e2e8f0; border-radius: 0.5rem; font-size: 1rem; box-sizing: border-box;",
                    }
                }

                // Debtor phone input (optional)
                div {
                    style: "margin-bottom: 1.5rem;",
                    label {
                        style: "display: block; font-size: 0.875rem; font-weight: 500; color: #4a5568; margin-bottom: 0.5rem;",
                        "Phone Number (Optional)"
                    }
                    input {
                        r#type: "tel",
                        placeholder: "Enter phone number",
                        value: "{debtor_phone}",
                        oninput: move |evt| on_phone_change.call(evt.value()),
                        style: "width: 100%; padding: 0.75rem; border: 2px solid #e2e8f0; border-radius: 0.5rem; font-size: 1rem; box-sizing: border-box;",
                    }
                }

                // Action buttons
                div {
                    style: "display: flex; gap: 0.75rem;",
                    button {
                        style: "flex: 1; background: #e2e8f0; color: #2d3748; padding: 0.75rem; border: none; border-radius: 0.5rem; cursor: pointer; font-weight: 500;",
                        onclick: move |_| on_cancel.call(()),
                        "Cancel"
                    }
                    button {
                        style: if is_valid {
                            "flex: 1; background: #ed8936; color: white; padding: 0.75rem; border: none; border-radius: 0.5rem; cursor: pointer; font-weight: 600;"
                        } else {
                            "flex: 1; background: #cbd5e0; color: #718096; padding: 0.75rem; border: none; border-radius: 0.5rem; cursor: not-allowed; font-weight: 600;"
                        },
                        disabled: !is_valid,
                        onclick: move |_| {
                            if is_valid {
                                on_confirm.call(());
                            }
                        },
                        "Create Loan & Complete Sale"
                    }
                }
            }
        }
    }
}
