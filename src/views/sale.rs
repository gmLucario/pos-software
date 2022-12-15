//! [`iced::Element`]s to be used in the sale view

use iced::{
    widget::{column, row, scrollable, text, text_input, Row},
    Alignment, Element, Length,
};
use sqlx::postgres::types::PgMoney;

use crate::{
    constants::{COLUMN_PADDING, SIZE_TEXT, SPACE_COLUMNS, TO_DECIMAL_DIGITS},
    helpers::{get_btn_cancel, get_btn_ok, get_btn_trash_icon},
    kinds::{AppEvents, SaleInputs},
    schemas::sale::{ProductList, ProductToAdd, SaleInfo},
};

/// Groups the different views Sale module has
pub struct SaleView {}

impl SaleView {
    /// View shows total to pay and payback money to client
    pub fn charge_sale_view(
        sale_info: &SaleInfo,
        is_pay_later: bool,
        is_ok_to_charge: bool,
    ) -> Element<AppEvents> {
        let mut ok_btn = get_btn_ok();
        if is_ok_to_charge {
            ok_btn = ok_btn.on_press(AppEvents::SaleCreateNewSale);
        }

        let pay_back_money = if is_pay_later {
            PgMoney(0)
        } else {
            sale_info.payback_money
        };

        let mut container = column!(
            text(format!(
                "Total: ${}",
                sale_info.total_pay.to_bigdecimal(TO_DECIMAL_DIGITS)
            ))
            .size(SIZE_TEXT),
            text_input("Pago:", &sale_info.client_pay, |input_value| {
                AppEvents::SaleInputChanged(input_value, SaleInputs::UserPay)
            })
            .on_submit(AppEvents::SaleCreateNewSale)
            .size(SIZE_TEXT)
            .width(Length::Units(100)),
            text(format!(
                "Cambio: ${}",
                pay_back_money.to_bigdecimal(TO_DECIMAL_DIGITS)
            ))
            .size(SIZE_TEXT),
        )
        .padding(60)
        .spacing(10)
        .align_items(Alignment::Center);

        if is_pay_later {
            container = container.push(
                text_input("Cliente:", &sale_info.client_name, |input_value| {
                    AppEvents::SaleInputChanged(input_value, SaleInputs::ClientName)
                })
                .on_submit(AppEvents::SaleCreateNewSale)
                .size(SIZE_TEXT)
                .width(Length::Units(500)),
            )
        }
        container = container.push(
            row!(
                get_btn_cancel().on_press(AppEvents::SaleNewProductCancel),
                ok_btn,
            )
            .spacing(20),
        );

        container.into()
    }

    /// View to show the form to add a new product to the sale list
    pub fn product_to_add_view(product: &ProductToAdd) -> Element<AppEvents> {
        column!(
            text(format!("Código Barras:  {}", product.barcode)).size(SIZE_TEXT),
            text(format!("Producto: {}", product.product_name)).size(SIZE_TEXT),
            text(format!("Precio: {}", product.price)).size(SIZE_TEXT),
            row!(
                text(format!("Cantidad [{}]:", product.unit_measurement)).size(SIZE_TEXT),
                text_input("", &product.amount, |input_value| {
                    AppEvents::SaleInputChanged(input_value, SaleInputs::AmountProduct)
                })
                .on_submit(AppEvents::SaleNewProductOk)
                .size(SIZE_TEXT)
                .width(Length::Units(100)),
            ),
            row!(
                get_btn_cancel().on_press(AppEvents::SaleNewProductCancel),
                get_btn_ok().on_press(AppEvents::SaleNewProductOk),
            )
            .spacing(20),
        )
        .padding(60)
        .spacing(10)
        .into()
    }

    /// Get titles of list products to be sold
    fn get_list_products_header<'a>() -> Row<'a, AppEvents> {
        row!(
            text("Producto:")
                .width(Length::FillPortion(5))
                .size(SIZE_TEXT),
            text("Cantidad:")
                .width(Length::FillPortion(2))
                .size(SIZE_TEXT),
            text("Precio:")
                .width(Length::FillPortion(2))
                .size(SIZE_TEXT),
            text("").size(SIZE_TEXT)
        )
    }

    /// Return a new row for the list of products to be sold
    fn format_product_row<'a>(product: &ProductList) -> Row<'a, AppEvents> {
        row!(
            text(product.product_name.to_string())
                .width(Length::FillPortion(5))
                .size(SIZE_TEXT),
            text(product.amount.to_string())
                .width(Length::FillPortion(2))
                .size(SIZE_TEXT),
            text(product.price.to_bigdecimal(TO_DECIMAL_DIGITS).to_string())
                .width(Length::FillPortion(2))
                .size(SIZE_TEXT),
        )
    }

    /// Shows main info current sale
    pub fn scan_barcodes_view(sale_info: &SaleInfo) -> Element<AppEvents> {
        let mut general_container = column!()
            .padding(COLUMN_PADDING)
            .spacing(SPACE_COLUMNS)
            .align_items(iced::Alignment::Center);

        let mut products_container =
            column!(Self::get_list_products_header()).spacing(SPACE_COLUMNS);

        let are_products: bool = !sale_info.products.is_empty();

        for (key, product) in sale_info.products.iter() {
            products_container = products_container.push(
                Self::format_product_row(product).push(
                    get_btn_trash_icon()
                        .on_press(AppEvents::SaleRemoveProductToBuyList(key.to_string())),
                ),
            );
        }

        let products_container = scrollable(products_container)
            // .width(Length::Fill)
            .height(Length::Fill);

        general_container = general_container.push(products_container).push(
            text(format!(
                "Total: ${}",
                sale_info.total_pay.to_bigdecimal(TO_DECIMAL_DIGITS)
            ))
            .size(SIZE_TEXT),
        );

        if are_products {
            general_container = general_container.push(
                row!(
                    get_btn_cancel().on_press(AppEvents::SaleProductsToBuyCancel),
                    get_btn_ok().on_press(AppEvents::SaleProductsToBuyOk)
                )
                .spacing(10),
            );
        }

        general_container.into()
    }
}
