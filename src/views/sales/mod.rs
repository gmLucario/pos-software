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
use crate::models::{LoanInput, Operation, Product, Sale, SaleInput, SaleItemInput};
use crate::views::loans::LoanForm;
use dioxus::prelude::*;
use rust_decimal::Decimal;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct CartItem {
    pub product: Rc<Product>,
    pub quantity: f64,
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

/// Parse payment amount from string input
fn parse_payment_amount(payment: &str) -> Result<Decimal, String> {
    if payment.is_empty() {
        Ok(Decimal::ZERO)
    } else {
        payment
            .parse::<Decimal>()
            .map_err(|_| "Invalid payment amount".to_string())
    }
}

/// Convert cart items to sale item inputs
fn cart_to_sale_items(cart_items: &[CartItem]) -> Vec<SaleItemInput> {
    cart_items
        .iter()
        .map(|item| SaleItemInput {
            product_id: item.product.id.clone(),
            product_name: item.product.full_name.clone(),
            quantity: item.quantity,
            unit_price: item.product.user_price,
        })
        .collect()
}

/// Signals needed for sale processing
struct SaleSignals {
    completed_sale: Signal<Option<(Sale, Vec<Operation>)>>,
    sale_message: Signal<Option<(bool, String)>>,
    cart: Signal<Vec<CartItem>>,
    payment_amount: Signal<String>,
    refresh_trigger: Signal<i32>,
}

/// Additional signals for loan sale processing
struct LoanSaleSignals {
    debtor_name: Signal<String>,
    debtor_phone: Signal<String>,
    show_loan_form: Signal<bool>,
}

/// Process a cash sale (non-loan)
fn process_cash_sale(
    app_state: AppState,
    cart_items: Vec<CartItem>,
    paid_amount: Decimal,
    mut signals: SaleSignals,
) {
    spawn(async move {
        let sale_input = SaleInput {
            items: cart_to_sale_items(&cart_items),
            paid_amount,
        };

        match execute_sale_transaction(app_state, sale_input, None).await {
            Ok((sale, operations)) => {
                signals.completed_sale.set(Some((sale, operations)));
                signals.cart.write().clear();
                signals.payment_amount.set(String::new());
                let current_trigger = *signals.refresh_trigger.read();
                signals.refresh_trigger.set(current_trigger + 1);
            }
            Err(err) => {
                signals.sale_message.set(Some((false, err)));
            }
        }
    });
}

/// Process a loan sale
fn process_loan_sale(
    app_state: AppState,
    cart_items: Vec<CartItem>,
    paid_amount: Decimal,
    loan_input: LoanInput,
    mut sale_signals: SaleSignals,
    mut loan_signals: LoanSaleSignals,
) {
    spawn(async move {
        let sale_input = SaleInput {
            items: cart_to_sale_items(&cart_items),
            paid_amount,
        };

        match execute_sale_transaction(app_state, sale_input, Some(loan_input)).await {
            Ok((sale, operations)) => {
                sale_signals.completed_sale.set(Some((sale, operations)));
                sale_signals.cart.write().clear();
                sale_signals.payment_amount.set(String::new());
                loan_signals.debtor_name.set(String::new());
                loan_signals.debtor_phone.set(String::new());
                loan_signals.show_loan_form.set(false);
                let current_trigger = *sale_signals.refresh_trigger.read();
                sale_signals.refresh_trigger.set(current_trigger + 1);
            }
            Err(err) => {
                sale_signals.sale_message.set(Some((false, err)));
                loan_signals.show_loan_form.set(false);
            }
        }
    });
}

/// Execute sale transaction (process sale, optionally create loan, fetch receipt)
async fn execute_sale_transaction(
    app_state: AppState,
    sale_input: SaleInput,
    loan_input: Option<LoanInput>,
) -> Result<(Sale, Vec<Operation>), String> {
    // Process sale
    let sale = app_state
        .sales_handler
        .process_sale(sale_input)
        .await
        .map_err(|e| format!("Sale failed: {}", e))?;

    // Create loan if needed
    if let Some(loan) = loan_input {
        app_state
            .loans_handler
            .create_loan(sale.id.clone(), loan)
            .await
            .map_err(|e| format!("Loan creation failed: {}", e))?;
    }

    // Fetch sale details for receipt
    let sale_details = app_state
        .sales_handler
        .get_sale_details(sale.id)
        .await
        .map_err(|e| format!("Failed to load receipt: {}", e))?;

    Ok((sale_details.sale, sale_details.operations))
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
    let refresh_trigger = use_signal(|| 0);
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
            cart_items.push(CartItem {
                product: Rc::new(product),
                quantity,
            });
        }

        // Close modal
        show_quantity_modal.set(None);
    };

    // Remove from cart
    let mut remove_from_cart = move |product_id: String| {
        cart.write().retain(|item| item.product.id != product_id);
    };

    // Clone app_state for closures
    let app_state_for_cash_sale = app_state.clone();
    let app_state_for_loan_sale = app_state.clone();

    // Check if sale is a loan and show form or complete sale
    let complete_sale = move |_| {
        if cart.read().is_empty() {
            sale_message.set(Some((false, "Cart is empty".to_string())));
            return;
        }

        let total = *cart_total.read();

        let paid_amount = match parse_payment_amount(&payment_amount.read()) {
            Ok(amount) => amount,
            Err(err) => {
                sale_message.set(Some((false, err)));
                return;
            }
        };

        // Check if this is a loan (payment < total)
        if paid_amount < total {
            show_loan_form.set(true);
        } else {
            process_cash_sale(
                app_state_for_cash_sale.clone(),
                cart.read().clone(),
                paid_amount,
                SaleSignals {
                    completed_sale,
                    sale_message,
                    cart,
                    payment_amount,
                    refresh_trigger,
                },
            );
        }
    };

    // Process loan sale after collecting debtor information
    let complete_loan_sale = move |_| {
        let paid_amount = match parse_payment_amount(&payment_amount.read()) {
            Ok(amount) => amount,
            Err(err) => {
                sale_message.set(Some((false, err)));
                return;
            }
        };

        let loan_input = LoanInput {
            sale_id: String::new(),
            debtor_name: debtor_name.read().to_string(),
            debtor_phone: {
                let phone = debtor_phone.read();
                if phone.trim().is_empty() {
                    None
                } else {
                    Some(phone.to_string())
                }
            },
        };

        process_loan_sale(
            app_state_for_loan_sale.clone(),
            cart.read().clone(),
            paid_amount,
            loan_input,
            SaleSignals {
                completed_sale,
                sale_message,
                cart,
                payment_amount,
                refresh_trigger,
            },
            LoanSaleSignals {
                debtor_name,
                debtor_phone,
                show_loan_form,
            },
        );
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
                    oninput: move |evt| search_query.set(evt.value()),
                    style: "width: 100%; padding: 0.75rem; border: 2px solid #e2e8f0; border-radius: 0.5rem; font-size: 1rem; margin-bottom: 1.5rem; box-sizing: border-box;",
                }

                // Products list - only shows when search is active
                {
                    let products = match &*products_resource.read_unchecked() {
                        Some(Ok(products)) => products.clone(),
                        _ => Vec::new(),
                    };
                    rsx! {
                        ProductsList {
                            products: Signal::new(products),
                            cart_items: cart,
                            search_query: search_query,
                            on_add: move |p: Product| show_product_modal(p),
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
                if let Some((is_success, message)) = sale_message.read().as_ref() {
                    SaleMessage {
                        is_success: *is_success,
                        message: message.clone(),
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
                    change_amount: change_amount,
                    payment_amount: payment_amount,
                    cart_is_empty: cart.read().is_empty(),
                    on_payment_change: move |value: String| payment_amount.set(value),
                    on_complete_sale: complete_sale,
                }
            }
        }

        // Quantity modal
        if let Some(product) = show_quantity_modal.read().as_ref() {
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
                debtor_name: debtor_name.read().to_string(),
                debtor_phone: debtor_phone.read().to_string(),
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
