//! Main Application Component
//!
//! The root component that handles navigation between modules.

use dioxus::prelude::*;

use super::{inventory, loans, sales};
use crate::handlers::AppState;

#[derive(Clone, Copy, PartialEq)]
pub enum ActiveTab {
    Inventory,
    Sales,
    Loans,
}

#[component]
pub fn App(app_state: AppState) -> Element {
    // Provide app state to all child components via context
    use_context_provider(|| app_state);

    // Global state: Current active tab
    let mut active_tab = use_signal(|| ActiveTab::Sales);

    rsx! {
        div {
            class: "app-container",
            style: "display: flex; flex-direction: column; height: 100vh; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;",

            // Header with navigation tabs
            header {
                class: "app-header",
                style: "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 1rem; box-shadow: 0 2px 8px rgba(0,0,0,0.1);",

                div {
                    style: "max-width: 1200px; margin: 0 auto;",

                    h1 {
                        style: "color: white; margin: 0 0 1rem 0; font-size: 1.8rem; font-weight: 600;",
                        "🏪 POS System"
                    }

                    // Tab navigation
                    nav {
                        class: "tab-navigation",
                        style: "display: flex; gap: 0.5rem;",

                        TabButton {
                            label: "💼 Sales",
                            is_active: *active_tab.read() == ActiveTab::Sales,
                            onclick: move |_| active_tab.set(ActiveTab::Sales),
                        }

                        TabButton {
                            label: "📦 Inventory",
                            is_active: *active_tab.read() == ActiveTab::Inventory,
                            onclick: move |_| active_tab.set(ActiveTab::Inventory),
                        }

                        TabButton {
                            label: "💰 Loans",
                            is_active: *active_tab.read() == ActiveTab::Loans,
                            onclick: move |_| active_tab.set(ActiveTab::Loans),
                        }
                    }
                }
            }

            // Main content area
            main {
                class: "app-main",
                style: "flex: 1; overflow: auto; background: #f5f7fa;",

                div {
                    style: "max-width: 1200px; margin: 0 auto; padding: 2rem;",

                    // Render active module
                    match *active_tab.read() {
                        ActiveTab::Inventory => rsx! {
                            inventory::InventoryView {}
                        },
                        ActiveTab::Sales => rsx! {
                            sales::SalesView {}
                        },
                        ActiveTab::Loans => rsx! {
                            loans::LoansView {}
                        },
                    }
                }
            }

            // Footer
            footer {
                class: "app-footer",
                style: "background: #2d3748; color: #a0aec0; padding: 1rem; text-align: center; font-size: 0.875rem;",

                "POS System v0.2.0 | Built with Dioxus 🚀"
            }
        }
    }
}

#[component]
fn TabButton(label: String, is_active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let base_style = "padding: 0.75rem 1.5rem; border: none; border-radius: 0.5rem; font-size: 1rem; font-weight: 500; cursor: pointer; transition: all 0.2s;";

    let style = if is_active {
        format!(
            "{} background: white; color: #667eea; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
            base_style
        )
    } else {
        format!(
            "{} background: rgba(255,255,255,0.2); color: white;",
            base_style
        )
    };

    rsx! {
        button {
            style: "{style}",
            onclick: move |evt| onclick.call(evt),
            onmouseenter: move |_| {},
            "{label}"
        }
    }
}
