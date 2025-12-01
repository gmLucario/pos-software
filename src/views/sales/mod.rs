//! Sales Module
//!
//! UI components for processing sales and managing the shopping cart.

use dioxus::prelude::*;
use rust_decimal::Decimal;
use crate::mock_data::MockProduct;

#[derive(Clone, Debug, PartialEq)]
struct CartItem {
    product: MockProduct,
    quantity: f64,
}

#[component]
pub fn SalesView(products: Vec<MockProduct>) -> Element {
    let mut cart = use_signal(Vec::<CartItem>::new);
    let mut search_query = use_signal(String::new);
    let mut payment_amount = use_signal(String::new);
    let mut show_receipt = use_signal(|| false);

    // Calculate cart total
    let cart_total: Decimal = cart.read().iter()
        .map(|item| item.product.price * Decimal::from_f64_retain(item.quantity).unwrap_or_default())
        .sum();

    // Filter products based on search
    let filtered_products = products.iter()
        .filter(|p| {
            let query = search_query.read().to_lowercase();
            if query.is_empty() {
                return true;
            }
            p.name.to_lowercase().contains(&query) ||
            p.barcode.as_ref().is_some_and(|b| b.contains(&query))
        })
        .cloned()
        .collect::<Vec<_>>();

    // Add product to cart
    let mut add_to_cart = move |product: MockProduct| {
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
        if cart.read().is_empty() {
            return;
        }
        show_receipt.set(true);
        // In real app, this would save to database
        cart.write().clear();
        payment_amount.set(String::new());
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

                // Products grid
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 1rem; max-height: 600px; overflow-y: auto;",

                    for product in filtered_products {
                        ProductCard {
                            product: product.clone(),
                            on_add: move |p: MockProduct| add_to_cart(p),
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
                        span { style: "font-weight: 600;", "${cart_total}" }
                    }

                    div {
                        style: "display: flex; justify-content: space-between; margin-bottom: 1rem; font-size: 1.5rem;",
                        span { style: "font-weight: 700;", "Total:" }
                        span { style: "font-weight: 700; color: #667eea;", "${cart_total}" }
                    }

                    // Payment input
                    div {
                        style: "margin-bottom: 1rem;",
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

        // Receipt modal (simplified for now)
        if *show_receipt.read() {
            div {
                style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;",
                onclick: move |_| show_receipt.set(false),

                div {
                    style: "background: white; padding: 2rem; border-radius: 0.5rem; max-width: 400px;",
                    onclick: move |evt| evt.stop_propagation(),

                    h3 { style: "margin: 0 0 1rem 0; color: #48bb78;", "✓ Sale Completed!" }
                    p { "Receipt would be generated here" }

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
fn ProductCard(product: MockProduct, on_add: EventHandler<MockProduct>) -> Element {
    let is_low_stock = product.is_low_stock();

    rsx! {
        div {
            style: "border: 2px solid #e2e8f0; border-radius: 0.5rem; padding: 1rem; cursor: pointer; transition: all 0.2s; background: white;",
            onclick: move |_| on_add.call(product.clone()),

            div {
                style: "font-weight: 600; margin-bottom: 0.5rem; color: #2d3748;",
                "{product.name}"
            }

            div {
                style: "font-size: 1.25rem; font-weight: 700; color: #667eea; margin-bottom: 0.5rem;",
                "${product.price}"
            }

            div {
                style: "font-size: 0.875rem; color: #718096;",
                "Stock: {product.stock} {product.unit}"
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
    let subtotal = item.product.price * Decimal::from_f64_retain(item.quantity).unwrap_or_default();

    rsx! {
        div {
            style: "padding: 0.75rem; border-bottom: 1px solid #e2e8f0; display: flex; justify-content: space-between; align-items: center;",

            div {
                style: "flex: 1;",
                div {
                    style: "font-weight: 500; color: #2d3748;",
                    "{item.product.name}"
                }
                div {
                    style: "font-size: 0.875rem; color: #718096;",
                    "{item.quantity} × ${item.product.price}"
                }
            }

            div {
                style: "display: flex; align-items: center; gap: 1rem;",
                div {
                    style: "font-weight: 600; color: #667eea;",
                    "${subtotal}"
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
