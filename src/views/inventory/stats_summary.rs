//! Stats Summary Component
//!
//! Displays inventory statistics in card format.

use crate::utils::formatting::format_currency;
use crate::views::inventory::helpers::InventoryStats;
use crate::views::inventory::stat_card::StatCard;
use dioxus::prelude::*;

#[component]
pub fn StatsSummary(stats: InventoryStats, is_search_mode: bool) -> Element {
    let (products_label, value_label) = get_labels(is_search_mode);

    rsx! {
        div {
            style: "margin-top: 1.5rem; padding-top: 1.5rem; border-top: 2px solid #e2e8f0; display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;",

            StatCard {
                label: products_label.to_string(),
                value: format!("{}", stats.total_count),
                color: "#667eea".to_string(),
            }

            StatCard {
                label: "Low Stock Items".to_string(),
                value: format!("{}", stats.low_stock_count),
                color: "#f56565".to_string(),
            }

            StatCard {
                label: value_label.to_string(),
                value: format_currency(stats.total_value),
                color: "#48bb78".to_string(),
            }
        }
    }
}

fn get_labels(is_search_mode: bool) -> (&'static str, &'static str) {
    if is_search_mode {
        ("Matching Products", "Search Value")
    } else {
        ("Total Products", "Total Value")
    }
}
