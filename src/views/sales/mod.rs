//! Sales Module
//!
//! UI components for processing sales and managing the shopping cart.

mod cart_item_row;
mod cart_summary;
mod product_card;
mod products_list;
mod quantity_modal;
mod receipt_template;
mod sale_message;
mod sale_receipt_modal;
mod validations;

pub use cart_item_row::CartItemRow;
pub use cart_summary::CartSummary;
pub use product_card::ProductCard;
pub use products_list::ProductsList;
pub use quantity_modal::QuantityModal;
pub use sale_message::SaleMessage;
use sale_receipt_modal::SaleReceiptModal;

use crate::handlers::AppState;
use crate::models::{Operation, Product, Sale, SaleInput, SaleItemInput};
use crate::views::loans::LoanForm;
use dioxus::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq)]
pub struct CartItem {
    pub product: Product,
    pub quantity: f64,
}

#[component]
pub fn SalesView() -> Element {
    // Get app state from context
    let app_state = use_context::<AppState>();

    let mut cart = use_signal(Vec::<CartItem>::new);
    let mut search_query = use_signal(String::new);
    let mut payment_amount = use_signal(String::new);
    let mut completed_sale = use_signal(|| Option::<(Sale, Vec<Operation>)>::None); // Completed sale with operations
    let mut sale_message = use_signal(|| Option::<(bool, String)>::None); // (is_success, message)
    let mut refresh_trigger = use_signal(|| 0);
    let mut show_quantity_modal = use_signal(|| Option::<Product>::None); // Product to add
    let mut show_loan_form = use_signal(|| false);
    let mut debtor_name = use_signal(String::new);
    let mut debtor_phone = use_signal(String::new);

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

    // Calculate cart total (reactive)
    let cart_total = use_memo(move || {
        cart.read()
            .iter()
            .map(|item| {
                item.product.user_price
                    * Decimal::from_f64_retain(item.quantity).unwrap_or_default()
            })
            .sum::<Decimal>()
    });

    // Calculate change (subtotal to return to buyer) - reactive
    let change_amount = use_memo(move || {
        let payment = payment_amount.read();
        let total = *cart_total.read();

        if payment.is_empty() {
            Decimal::ZERO
        } else {
            match payment.parse::<Decimal>() {
                Ok(paid) => {
                    if paid > total {
                        paid - total
                    } else {
                        Decimal::ZERO
                    }
                }
                Err(_) => Decimal::ZERO,
            }
        }
    });

    // Show quantity modal for product
    let mut show_product_modal = move |product: Product| {
        show_quantity_modal.set(Some(product));
    };

    // Add product to cart with specified quantity
    let add_to_cart_with_quantity = move |(product, quantity): (Product, f64)| {
        let cart_items_read = cart.read();

        // Check if quantity exceeds available stock
        let existing_quantity = cart_items_read
            .iter()
            .find(|item| item.product.id == product.id)
            .map(|item| item.quantity)
            .unwrap_or(0.0);

        let total_quantity = existing_quantity + quantity;

        if total_quantity > product.current_amount {
            sale_message.set(Some((
                false,
                format!(
                    "Cannot add {}. Only {} available (already have {} in cart)",
                    quantity, product.current_amount, existing_quantity
                ),
            )));
            show_quantity_modal.set(None);
            return;
        }

        // Drop the read guard before writing
        drop(cart_items_read);
        let mut cart_items = cart.write();

        // Add or update cart item
        if let Some(item) = cart_items.iter_mut().find(|i| i.product.id == product.id) {
            item.quantity += quantity;
        } else {
            cart_items.push(CartItem { product, quantity });
        }

        // Close modal
        show_quantity_modal.set(None);
    };

    // Remove from cart
    let mut remove_from_cart = move |product_id: String| {
        cart.write().retain(|item| item.product.id != product_id);
    };

    // Clone app_state for closures
    let app_state_for_sale = app_state.clone();
    let app_state_for_loan = app_state.clone();

    // Check if sale is a loan and show form or complete sale
    let complete_sale = move |_| {
        let cart_items = cart.read().clone();
        let payment = payment_amount.read().clone();
        let total = *cart_total.read();

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

        // Check if this is a loan (payment < total)
        if paid_amount < total {
            // Show loan form to collect debtor information
            show_loan_form.set(true);
        } else {
            // Process cash sale directly
            let app_state = app_state_for_sale.clone();
            spawn(async move {
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

                match app_state.sales_handler.process_sale(sale_input).await {
                    Ok(sale) => {
                        // Fetch sale details with operations to show receipt
                        match app_state.sales_handler.get_sale_details(sale.id).await {
                            Ok(sale_with_ops) => {
                                completed_sale
                                    .set(Some((sale_with_ops.sale, sale_with_ops.operations)));
                                cart.write().clear();
                                payment_amount.set(String::new());
                                refresh_trigger.set(refresh_trigger() + 1);
                            }
                            Err(err) => {
                                sale_message
                                    .set(Some((false, format!("Failed to load receipt: {}", err))));
                            }
                        }
                    }
                    Err(err) => {
                        sale_message.set(Some((false, format!("Sale failed: {}", err))));
                    }
                }
            });
        }
    };

    // Process loan sale after collecting debtor information
    let complete_loan_sale = move |_| {
        let app_state = app_state_for_loan.clone();
        let cart_items = cart.read().clone();
        let payment = payment_amount.read().clone();
        let debtor_name_value = debtor_name.read().clone();
        let debtor_phone_value = debtor_phone.read().clone();

        spawn(async move {
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
                Ok(sale) => {
                    // Create loan with debtor information
                    use crate::models::LoanInput;
                    let loan_input = LoanInput {
                        sale_id: sale.id.clone(),
                        debtor_name: debtor_name_value,
                        debtor_phone: if debtor_phone_value.trim().is_empty() {
                            None
                        } else {
                            Some(debtor_phone_value)
                        },
                    };

                    match app_state
                        .loans_handler
                        .create_loan(sale.id.clone(), loan_input)
                        .await
                    {
                        Ok(_loan) => {
                            // Fetch sale details with operations to show receipt
                            match app_state.sales_handler.get_sale_details(sale.id).await {
                                Ok(sale_with_ops) => {
                                    completed_sale
                                        .set(Some((sale_with_ops.sale, sale_with_ops.operations)));
                                    cart.write().clear();
                                    payment_amount.set(String::new());
                                    debtor_name.set(String::new());
                                    debtor_phone.set(String::new());
                                    show_loan_form.set(false);
                                    refresh_trigger.set(refresh_trigger() + 1);
                                }
                                Err(err) => {
                                    sale_message.set(Some((
                                        false,
                                        format!("Failed to load receipt: {}", err),
                                    )));
                                    show_loan_form.set(false);
                                }
                            }
                        }
                        Err(err) => {
                            sale_message
                                .set(Some((false, format!("Loan creation failed: {}", err))));
                            show_loan_form.set(false);
                        }
                    }
                }
                Err(err) => {
                    sale_message.set(Some((false, format!("Sale failed: {}", err))));
                    show_loan_form.set(false);
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
                    Some(Ok(products)) => rsx! {
                        ProductsList {
                            products: products.clone(),
                            cart_items: cart.read().clone(),
                            search_query: search_query.read().clone(),
                            on_add: move |p: Product| show_product_modal(p),
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
                    SaleMessage {
                        is_success,
                        message,
                        on_dismiss: move |_| sale_message.set(None),
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
                CartSummary {
                    cart_total: *cart_total.read(),
                    change_amount: *change_amount.read(),
                    payment_amount: payment_amount.read().clone(),
                    cart_is_empty: cart.read().is_empty(),
                    on_payment_change: move |value: String| payment_amount.set(value),
                    on_complete_sale: complete_sale,
                }
            }
        }

        // Quantity modal
        if let Some(product) = show_quantity_modal.read().clone() {
            QuantityModal {
                product: product.clone(),
                unit_abbreviation: get_unit_abbreviation(product.unit_measurement_id).to_string(),
                on_confirm: add_to_cart_with_quantity,
                on_cancel: move |_| show_quantity_modal.set(None),
            }
        }

        // Loan form modal
        if *show_loan_form.read() {
            LoanForm {
                debtor_name: debtor_name.read().clone(),
                debtor_phone: debtor_phone.read().clone(),
                on_name_change: move |value: String| debtor_name.set(value),
                on_phone_change: move |value: String| debtor_phone.set(value),
                on_cancel: move |_| show_loan_form.set(false),
                on_confirm: complete_loan_sale,
            }
        }

        // Receipt modal
        if let Some((sale, operations)) = completed_sale.read().as_ref() {
            SaleReceiptModal {
                sale: sale.clone(),
                operations: operations.clone(),
                on_close: move |_| completed_sale.set(None),
            }
        }
    }
}

/// Get unit measurement abbreviation from ID
fn get_unit_abbreviation(unit_id: i32) -> &'static str {
    use crate::models::UnitMeasurement;
    match unit_id {
        UnitMeasurement::KILOGRAM => "kg",
        UnitMeasurement::LITER => "lt",
        UnitMeasurement::UNIT => "unit",
        UnitMeasurement::PIECE => "pcs",
        UnitMeasurement::BOX => "box",
        UnitMeasurement::CAN => "can",
        UnitMeasurement::BOTTLE => "btl",
        _ => "unit",
    }
}
