//! Sales Module
//!
//! UI components for processing sales and managing the shopping cart.

use crate::handlers::AppState;
use crate::models::{Product, SaleInput, SaleItemInput};
use crate::utils::formatting::format_currency;
use dioxus::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq)]
struct CartItem {
    product: Product,
    quantity: f64,
}

#[component]
pub fn SalesView() -> Element {
    // Get app state from context
    let app_state = use_context::<AppState>();

    let mut cart = use_signal(Vec::<CartItem>::new);
    let mut search_query = use_signal(String::new);
    let mut payment_amount = use_signal(String::new);
    let mut show_receipt = use_signal(|| false);
    let mut sale_message = use_signal(|| Option::<(bool, String)>::None); // (is_success, message)
    let mut refresh_trigger = use_signal(|| 0);

    // Load products from database
    let mut products_resource = use_resource({
        let inventory_handler = app_state.inventory_handler.clone();
        move || {
            let handler = inventory_handler.clone();
            async move { handler.load_products().await }
        }
    });

    // Refresh products when trigger changes
    use_effect(move || {
        let _ = refresh_trigger();
        products_resource.restart();
    });

    // Calculate cart total
    let cart_total: Decimal = cart
        .read()
        .iter()
        .map(|item| {
            item.product.user_price * Decimal::from_f64_retain(item.quantity).unwrap_or_default()
        })
        .sum();

    // Add product to cart
    let mut add_to_cart = move |product: Product| {
        let mut cart_items = cart.write();

        // Check if product already in cart
        if let Some(item) = cart_items.iter_mut().find(|i| i.product.id == product.id) {
            item.quantity += 1.0;
        } else {
            cart_items.push(CartItem {
                product,
                quantity: 1.0,
            });
        }
    };

    // Remove from cart
    let mut remove_from_cart = move |product_id: String| {
        cart.write().retain(|item| item.product.id != product_id);
    };

    // Complete sale
    let complete_sale = move |_| {
        let app_state = app_state.clone();
        let cart_items = cart.read().clone();
        let payment = payment_amount.read().clone();

        spawn(async move {
            if cart_items.is_empty() {
                sale_message.set(Some((false, "Cart is empty".to_string())));
                return;
            }

            // Parse payment amount
            let paid_amount = if payment.is_empty() {
                Decimal::ZERO
            } else {
                match payment.parse::<Decimal>() {
                    Ok(amount) => amount,
                    Err(_) => {
                        sale_message.set(Some((false, "Invalid payment amount".to_string())));
                        return;
                    }
                }
            };

            // Determine if this is a cash sale or loan
            let _cart_total: Decimal = cart_items
                .iter()
                .map(|item| {
                    item.product.user_price
                        * Decimal::from_f64_retain(item.quantity).unwrap_or_default()
                })
                .sum();

            // Create sale input
            let sale_input = SaleInput {
                items: cart_items
                    .iter()
                    .map(|item| SaleItemInput {
                        product_id: item.product.id.clone(),
                        product_name: item.product.full_name.clone(),
                        quantity: item.quantity,
                        unit_price: item.product.user_price,
                    })
                    .collect(),
                paid_amount,
            };

            // Process sale
            match app_state.sales_handler.process_sale(sale_input).await {
                Ok(_sale) => {
                    sale_message.set(Some((true, "Sale completed successfully!".to_string())));
                    show_receipt.set(true);
                    cart.write().clear();
                    payment_amount.set(String::new());
                    refresh_trigger.set(refresh_trigger() + 1); // Refresh product stock
                }
                Err(err) => {
                    sale_message.set(Some((false, format!("Sale failed: {}", err))));
                }
            }
        });
    };

    rsx! {
        div {
            class: "sales-view",
            style: "display: grid; grid-template-columns: 1fr 400px; gap: 1.5rem;",

            // Left side: Product selection
            div {
                style: "background: white; border-radius: 0.5rem; padding: 1.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",

                h2 {
                    style: "font-size: 1.5rem; font-weight: 600; color: #2d3748; margin: 0 0 1.5rem 0;",
                    "💼 New Sale"
                }

                // Product search
                input {
                    r#type: "text",
                    placeholder: "🔍 Search products or scan barcode...",
                    value: "{search_query}",
                    oninput: move |evt| search_query.set(evt.value().clone()),
                    style: "width: 100%; padding: 0.75rem; border: 2px solid #e2e8f0; border-radius: 0.5rem; font-size: 1rem; margin-bottom: 1.5rem; box-sizing: border-box;",
                }

                // Products based on loading state
                match &*products_resource.read_unchecked() {
                    Some(Ok(products)) => {
                        // Filter products
                        let filtered_products: Vec<Product> = products.iter()
                            .filter(|p| {
                                let query = search_query.read().to_lowercase();
                                if query.is_empty() {
                                    return true;
                                }
                                p.full_name.to_lowercase().contains(&query) ||
                                p.barcode.as_ref().is_some_and(|b| b.contains(&query))
                            })
                            .cloned()
                            .collect();

                        rsx! {
                            div {
                                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 1rem; max-height: 600px; overflow-y: auto;",

                                if filtered_products.is_empty() {
                                    div {
                                        style: "grid-column: 1 / -1; padding: 2rem; text-align: center; color: #718096;",
                                        "No products found"
                                    }
                                } else {
                                    for product in filtered_products {
                                        ProductCard {
                                            product: product.clone(),
                                            on_add: move |p: Product| add_to_cart(p),
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(err)) => rsx! {
                        div {
                            style: "padding: 2rem; text-align: center; color: #e53e3e; background: #fff5f5; border-radius: 0.5rem;",
                            "❌ Error loading products: {err}"
                        }
                    },
                    None => rsx! {
                        div {
                            style: "padding: 2rem; text-align: center; color: #718096;",
                            "⏳ Loading products..."
                        }
                    }
                }
            }

            // Right side: Shopping cart
            div {
                style: "background: white; border-radius: 0.5rem; padding: 1.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1); display: flex; flex-direction: column; max-height: calc(100vh - 200px);",

                h3 {
                    style: "font-size: 1.25rem; font-weight: 600; color: #2d3748; margin: 0 0 1rem 0;",
                    "🛒 Cart ({cart.read().len()} items)"
                }

                // Sale message
                if let Some((is_success, message)) = sale_message.read().clone() {
                    div {
                        style: if is_success {
                            "padding: 0.75rem; margin-bottom: 1rem; background: #f0fff4; color: #22543d; border-radius: 0.5rem; border: 1px solid #48bb78;"
                        } else {
                            "padding: 0.75rem; margin-bottom: 1rem; background: #fff5f5; color: #c53030; border-radius: 0.5rem; border: 1px solid #f56565;"
                        },
                        "{message}"
                        button {
                            style: "float: right; background: transparent; border: none; cursor: pointer; font-weight: bold;",
                            onclick: move |_| sale_message.set(None),
                            "✕"
                        }
                    }
                }

                // Cart items
                div {
                    style: "flex: 1; overflow-y: auto; margin-bottom: 1rem;",

                    if cart.read().is_empty() {
                        div {
                            style: "text-align: center; padding: 3rem; color: #a0aec0;",
                            p { "Cart is empty" }
                            p { style: "font-size: 3rem;", "🛒" }
                        }
                    } else {
                        for item in cart.read().iter() {
                            CartItemRow {
                                item: item.clone(),
                                on_remove: move |id: String| remove_from_cart(id),
                            }
                        }
                    }
                }

                // Cart summary
                div {
                    style: "border-top: 2px solid #e2e8f0; padding-top: 1rem;",

                    div {
                        style: "display: flex; justify-content: space-between; margin-bottom: 0.5rem; font-size: 1.125rem;",
                        span { style: "font-weight: 500;", "Subtotal:" }
                        span { style: "font-weight: 600;", "{format_currency(cart_total)}" }
                    }

                    div {
                        style: "display: flex; justify-content: space-between; margin-bottom: 1rem; font-size: 1.5rem;",
                        span { style: "font-weight: 700;", "Total:" }
                        span { style: "font-weight: 700; color: #667eea;", "{format_currency(cart_total)}" }
                    }

                    // Payment input
                    div {
                        style: "margin-bottom: 1rem;",
                        label {
                            style: "display: block; font-size: 0.875rem; font-weight: 500; color: #4a5568; margin-bottom: 0.5rem;",
                            "Payment Amount (leave empty for loan)"
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

                    // Complete sale button
                    button {
                        style: "width: 100%; background: #48bb78; color: white; padding: 1rem; border: none; border-radius: 0.5rem; font-size: 1.125rem; font-weight: 600; cursor: pointer; transition: background 0.2s;",
                        disabled: cart.read().is_empty(),
                        onclick: complete_sale,
                        "💳 Complete Sale"
                    }
                }
            }
        }

        // Receipt modal
        if *show_receipt.read() {
            div {
                style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;",
                onclick: move |_| show_receipt.set(false),

                div {
                    style: "background: white; padding: 2rem; border-radius: 0.5rem; max-width: 400px;",
                    onclick: move |evt| evt.stop_propagation(),

                    h3 { style: "margin: 0 0 1rem 0; color: #48bb78;", "✓ Sale Completed!" }
                    p { "The sale has been recorded successfully." }
                    p { style: "font-size: 0.875rem; color: #718096;", "Inventory has been updated automatically." }

                    button {
                        style: "width: 100%; background: #667eea; color: white; padding: 0.75rem; border: none; border-radius: 0.5rem; cursor: pointer;",
                        onclick: move |_| show_receipt.set(false),
                        "Close"
                    }
                }
            }
        }
    }
}

#[component]
fn ProductCard(product: Product, on_add: EventHandler<Product>) -> Element {
    let is_low_stock = product.is_low_stock();

    rsx! {
        div {
            style: "border: 2px solid #e2e8f0; border-radius: 0.5rem; padding: 1rem; cursor: pointer; transition: all 0.2s; background: white;",
            onclick: move |_| on_add.call(product.clone()),

            div {
                style: "font-weight: 600; margin-bottom: 0.5rem; color: #2d3748;",
                "{product.full_name}"
            }

            div {
                style: "font-size: 1.25rem; font-weight: 700; color: #667eea; margin-bottom: 0.5rem;",
                "{format_currency(product.user_price)}"
            }

            div {
                style: "font-size: 0.875rem; color: #718096;",
                "Stock: {product.current_amount:.2}"
            }

            if is_low_stock {
                div {
                    style: "margin-top: 0.5rem; background: #fff5f5; color: #c53030; padding: 0.25rem 0.5rem; border-radius: 0.25rem; font-size: 0.75rem; text-align: center;",
                    "⚠️ Low Stock"
                }
            }
        }
    }
}

#[component]
fn CartItemRow(item: CartItem, on_remove: EventHandler<String>) -> Element {
    let subtotal =
        item.product.user_price * Decimal::from_f64_retain(item.quantity).unwrap_or_default();

    rsx! {
        div {
            style: "padding: 0.75rem; border-bottom: 1px solid #e2e8f0; display: flex; justify-content: space-between; align-items: center;",

            div {
                style: "flex: 1;",
                div {
                    style: "font-weight: 500; color: #2d3748;",
                    "{item.product.full_name}"
                }
                div {
                    style: "font-size: 0.875rem; color: #718096;",
                    "{item.quantity} × {format_currency(item.product.user_price)}"
                }
            }

            div {
                style: "display: flex; align-items: center; gap: 1rem;",
                div {
                    style: "font-weight: 600; color: #667eea;",
                    "{format_currency(subtotal)}"
                }
                button {
                    style: "background: #f56565; color: white; border: none; border-radius: 0.25rem; padding: 0.25rem 0.5rem; cursor: pointer; font-size: 0.875rem;",
                    onclick: move |_| on_remove.call(item.product.id.clone()),
                    "✕"
                }
            }
        }
    }
}
