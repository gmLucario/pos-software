//! User interfaces shown in sale module

use iced::{
    widget::{column, container, horizontal_rule, row, scrollable, text, text_input},
    Alignment, Element, Length,
};
use sqlx::postgres::types::PgMoney;

use crate::{
    constants::{
        COLUMN_PADDING, PGMONEY_DECIMALS, SIZE_TEXT, SPACE_COLUMNS, SPACE_ROWS,
        SPACE_ROW_BTNS_FORM, TO_DECIMAL_DIGITS,
    },
    events::AppEvent,
    helpers::{get_btn_cancel, get_btn_edit, get_btn_ok, get_btn_trash_icon},
    kinds::{TextInput, UnitsMeasurement},
    models::sale::SaleProductInfo,
    schemas::sale::SaleInfo,
};

/// Shows main info current sale
pub fn products_to_be_sold(
    sale_info: &SaleInfo,
    is_pay_later: bool,
    is_ok_to_charge: bool,
) -> Element<AppEvent> {
    let products: Vec<Element<AppEvent>> = sale_info
        .products
        .iter()
        .map(|(barcode, product)| format_product_list_to_be_sold(barcode.to_string(), product))
        .collect();

    let are_products = !products.is_empty();
    let (mut cancel_btn, mut ok_btn) = (get_btn_cancel(), get_btn_ok());

    if are_products {
        if is_ok_to_charge {
            ok_btn = ok_btn.on_press(AppEvent::SaleCreateNewSale);
        }
        cancel_btn = cancel_btn.on_press(AppEvent::SaleResetProductsToBeSold);
    }

    column![
        scrollable(column(products).spacing(SPACE_COLUMNS)).height(Length::Fill),
        horizontal_rule(2),
        container(
            column![
                text(format!(
                    "Total: ${}",
                    sale_info.total_pay.to_bigdecimal(TO_DECIMAL_DIGITS)
                ))
                .size(SIZE_TEXT),
                container(charge_sale_view(sale_info, is_pay_later)),
                row!(cancel_btn, ok_btn).spacing(SPACE_ROW_BTNS_FORM)
            ]
            .align_items(Alignment::End)
        )
    ]
    .align_items(Alignment::Center)
    .width(Length::Fill)
    .spacing(SPACE_COLUMNS)
    .padding(COLUMN_PADDING)
    .into()
}

/// Format each row in products to be sold list
fn format_product_list_to_be_sold(barcode: String, product: &SaleProductInfo) -> Element<AppEvent> {
    let total_price = PgMoney::from_bigdecimal(
        &product.amount * product.price.to_bigdecimal(TO_DECIMAL_DIGITS),
        PGMONEY_DECIMALS,
    )
    .unwrap()
    .to_bigdecimal(TO_DECIMAL_DIGITS);

    let label = format!(
        "-| {amount}[{unit_measurement}] {product_name} (${price_per_unit}): ",
        amount = product.amount,
        unit_measurement = UnitsMeasurement::from(product.unit_measurement_id),
        product_name = product.product_name,
        price_per_unit = product.price.to_bigdecimal(TO_DECIMAL_DIGITS),
    );
    let label = text(label).size(SIZE_TEXT).width(Length::Fill);
    let total_price_label = text(format!("${}", total_price)).size(SIZE_TEXT);

    row!(
        label,
        total_price_label,
        get_btn_edit().on_press(AppEvent::SaleEditProductToBeSold(product.clone())),
        get_btn_trash_icon().on_press(AppEvent::SaleRemoveProductToBeSold(barcode)),
    )
    .spacing(SPACE_ROWS)
    .into()
}

/// Show the form to add/edit a new product to the sale list
pub fn product_to_add_form(
    product: &SaleProductInfo,
    user_input_amount: String,
    is_edit: bool,
) -> Element<AppEvent> {
    column!(
        text(format!("Código Barras:  {}", product.barcode)).size(SIZE_TEXT),
        text(format!("Producto: {}", product.product_name)).size(SIZE_TEXT),
        text(format!(
            "Precio: {}",
            product.price.to_bigdecimal(TO_DECIMAL_DIGITS)
        ))
        .size(SIZE_TEXT),
        row!(
            text(format!(
                "Cantidad [{}]:",
                UnitsMeasurement::from(product.unit_measurement_id)
            ))
            .size(SIZE_TEXT),
            text_input("", &user_input_amount)
                .on_input(|input_value| {
                    AppEvent::TextInputChanged(
                        input_value,
                        TextInput::SaleFormProductAmount(UnitsMeasurement::from(
                            product.unit_measurement_id,
                        )),
                    )
                })
                .on_submit(AppEvent::SaleProductAddEditFormOk(product.clone(), is_edit))
                .size(SIZE_TEXT)
                .width(Length::from(100)),
        ),
    )
    .padding(COLUMN_PADDING)
    .spacing(SPACE_COLUMNS)
    .into()
}

/// View shows total to pay and payback money to client
fn charge_sale_view(sale_info: &SaleInfo, is_pay_later: bool) -> Element<AppEvent> {
    let pay_back_money = if is_pay_later {
        PgMoney(0)
    } else {
        sale_info.payback_money
    };

    let are_products = !sale_info.products.is_empty();
    let user_payment = text_input("Pago:", &sale_info.client_pay).size(SIZE_TEXT);
    let user_payment = match are_products {
        true => user_payment
            .on_input(|input_value| {
                AppEvent::TextInputChanged(input_value, TextInput::SaleUserPayment)
            })
            .on_submit(AppEvent::SaleCreateNewSale),
        false => user_payment,
    };

    let mut container = column!(
        user_payment,
        text(format!(
            "Cambio: ${}",
            pay_back_money.to_bigdecimal(TO_DECIMAL_DIGITS)
        ))
        .size(SIZE_TEXT),
    )
    .spacing(SPACE_COLUMNS)
    .padding(COLUMN_PADDING);

    if is_pay_later {
        container = container.push(
            text_input("Cliente:", &sale_info.client_name)
                .on_input(|input_value| {
                    AppEvent::TextInputChanged(input_value, TextInput::SaleClientNameLoan)
                })
                .on_submit(AppEvent::SaleCreateNewSale)
                .size(SIZE_TEXT),
        )
    }

    container.into()
}
