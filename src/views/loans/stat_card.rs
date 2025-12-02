//! Stat Card Component
//!
//! Displays a statistic card with label, value, color, and icon.

use dioxus::prelude::*;

#[component]
pub fn StatCard(label: String, value: String, color: String, icon: String) -> Element {
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
