//! Sale Message Component
//!
//! Displays success or error messages for sale operations.

use dioxus::prelude::*;

#[component]
pub fn SaleMessage(is_success: bool, message: String, on_dismiss: EventHandler<()>) -> Element {
    let (bg_color, text_color, border_color) = if is_success {
        (
            "#f0fff4", // green background
            "#22543d", // dark green text
            "#48bb78", // green border
        )
    } else {
        (
            "#fff5f5", // red background
            "#c53030", // dark red text
            "#f56565", // red border
        )
    };

    rsx! {
        div {
            style: "padding: 0.75rem; margin-bottom: 1rem; background: {bg_color}; color: {text_color}; border-radius: 0.5rem; border: 1px solid {border_color};",
            "{message}"
            button {
                style: "float: right; background: transparent; border: none; cursor: pointer; font-weight: bold;",
                onclick: move |_| on_dismiss.call(()),
                "✕"
            }
        }
    }
}
