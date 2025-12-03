//! Pagination Navigation Component
//!
//! Provides navigation controls for paginated loan listings.

use dioxus::prelude::*;

#[component]
pub fn PaginationNav(current_page: Signal<i64>, total_pages: i64) -> Element {
    rsx! {
        div {
            style: "margin-top: 1.5rem; display: flex; justify-content: center; align-items: center; gap: 1rem;",

            button {
                style: format!(
                    "padding: 0.5rem 1rem; border: 1px solid #e2e8f0; background: {}; border-radius: 0.5rem; cursor: {}; font-size: 0.875rem; font-weight: 500; color: {};",
                    if current_page() > 1 { "white" } else { "#f7fafc" },
                    if current_page() > 1 { "pointer" } else { "not-allowed" },
                    if current_page() > 1 { "#4a5568" } else { "#cbd5e0" }
                ),
                disabled: current_page() <= 1,
                onclick: move |_| {
                    if current_page() > 1 {
                        current_page.set(current_page() - 1);
                    }
                },
                "← Previous"
            }

            span {
                style: "font-size: 0.875rem; color: #4a5568;",
                "Page {current_page()} of {total_pages}"
            }

            button {
                style: format!(
                    "padding: 0.5rem 1rem; border: 1px solid #e2e8f0; background: {}; border-radius: 0.5rem; cursor: {}; font-size: 0.875rem; font-weight: 500; color: {};",
                    if current_page() < total_pages { "white" } else { "#f7fafc" },
                    if current_page() < total_pages { "pointer" } else { "not-allowed" },
                    if current_page() < total_pages { "#4a5568" } else { "#cbd5e0" }
                ),
                disabled: current_page() >= total_pages,
                onclick: move |_| {
                    if current_page() < total_pages {
                        current_page.set(current_page() + 1);
                    }
                },
                "Next →"
            }
        }
    }
}
