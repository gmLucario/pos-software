//! Stat Card Component
//!
//! Displays a statistic in a styled card format.

use dioxus::prelude::*;

#[component]
pub fn StatCard(label: String, value: String, color: String) -> Element {
    rsx! {
        div {
            style: "background: #f7fafc; padding: 1rem; border-radius: 0.5rem; border-left: 4px solid {color};",

            div {
                style: "font-size: 0.875rem; color: #718096; margin-bottom: 0.5rem;",
                "{label}"
            }
            div {
                style: "font-size: 1.5rem; font-weight: 700; color: {color};",
                "{value}"
            }
        }
    }
}
