use iced::{Alignment, Button, Column, Element, Length, Row, Scrollable, Text, TextInput};
use sqlx::postgres::types::PgMoney;

use crate::{
    constants::{SIZE_TEXT, SPACE_COLUMNS, TO_DECIMAL_DIGITS},
    controllers,
    kinds::{AppEvents, SaleInputs},
    schemas::sale::ProductList,
};

use super::fonts;

impl controllers::sale::Sale {
    pub fn charge_sale_view(&mut self) -> Element<AppEvents> {
        let is_pay_later = self.is_pay_later();
        let is_ok_to_charge = self.is_ok_charge();

        let mut ok_btn = Button::new(&mut self.ok_new_record_btn_state, Text::new("Ok"));
        if is_ok_to_charge {
            ok_btn = ok_btn.on_press(AppEvents::SaleCreateNewSale);
        }

        let pay_back_money = if is_pay_later {
            PgMoney(0)
        } else {
            self.sale_info.payback_money
        };

        let mut container = Column::new()
            .padding(60)
            .spacing(10)
            .align_items(Alignment::Center)
            .push(
                Text::new(format!(
                    "Total: {}",
                    self.sale_info.total_pay.to_bigdecimal(TO_DECIMAL_DIGITS)
                ))
                .size(SIZE_TEXT),
            )
            .push(
                TextInput::new(
                    &mut self.client_pay_input_state,
                    "Pago:",
                    &self.sale_info.client_pay,
                    |input_value| AppEvents::SaleInputChanged(input_value, SaleInputs::UserPay),
                )
                .on_submit(AppEvents::SaleCreateNewSale)
                .size(SIZE_TEXT)
                .width(Length::Units(100)),
            )
            .push(
                Text::new(format!(
                    "Cambio: {}",
                    pay_back_money.to_bigdecimal(TO_DECIMAL_DIGITS)
                ))
                .size(SIZE_TEXT),
            );

        if is_pay_later {
            container = container.push(
                TextInput::new(
                    &mut self.client_name_input_state,
                    "Cliente:",
                    &self.sale_info.client_name,
                    |input_value| AppEvents::SaleInputChanged(input_value, SaleInputs::ClientName),
                )
                .on_submit(AppEvents::SaleCreateNewSale)
                .size(SIZE_TEXT)
                .width(Length::Units(500)),
            )
        }
        container = container.push(
            Row::new()
                .push(
                    Button::new(&mut self.cancel_new_record_btn_state, Text::new("Cancelar"))
                        .on_press(AppEvents::SaleNewProductCancel),
                )
                .push(ok_btn)
                .spacing(20),
        );

        container.into()
    }

    pub fn product_to_add_view(&mut self) -> Element<AppEvents> {
        Column::new()
            .padding(60)
            .spacing(10)
            .push(
                Text::new(format!("Código Barras:  {}", self.product_to_add.barcode))
                    .size(SIZE_TEXT),
            )
            .push(
                Text::new(format!("Producto: {}", self.product_to_add.product_name))
                    .size(SIZE_TEXT),
            )
            .push(Text::new(format!("Precio: {}", self.product_to_add.price)).size(SIZE_TEXT))
            .push(
                Row::new()
                    .push(
                        Text::new(format!(
                            "Cantidad [{}]:",
                            self.product_to_add.unit_measurement
                        ))
                        .size(SIZE_TEXT),
                    )
                    .push(
                        TextInput::new(
                            &mut self.amount_input_state,
                            "",
                            &self.product_to_add.amount,
                            |input_value| {
                                AppEvents::SaleInputChanged(input_value, SaleInputs::AmountProduct)
                            },
                        )
                        .on_submit(AppEvents::SaleNewProductOk)
                        .size(SIZE_TEXT)
                        .width(Length::Units(100)),
                    ),
            )
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.cancel_new_record_btn_state, Text::new("Cancelar"))
                            .on_press(AppEvents::SaleNewProductCancel),
                    )
                    .push(
                        Button::new(&mut self.ok_new_record_btn_state, Text::new("Ok"))
                            .on_press(AppEvents::SaleNewProductOk),
                    )
                    .spacing(20),
            )
            .into()
    }

    fn get_list_products_header<'a>() -> Row<'a, AppEvents> {
        Row::new()
            .push(
                Text::new("Producto:")
                    .width(Length::FillPortion(5))
                    .size(SIZE_TEXT),
            )
            .push(
                Text::new("Cantidad:")
                    .width(Length::FillPortion(2))
                    .size(SIZE_TEXT),
            )
            .push(
                Text::new("Precio:")
                    .width(Length::FillPortion(2))
                    .size(SIZE_TEXT),
            )
            .push(Text::new("").size(SIZE_TEXT))
    }

    fn format_product_row<'a>(product: &ProductList) -> Row<'a, AppEvents> {
        Row::new()
            .push(
                Text::new(product.product_name.to_string())
                    .width(Length::FillPortion(5))
                    .size(SIZE_TEXT),
            )
            .push(
                Text::new(product.amount.to_string())
                    .width(Length::FillPortion(2))
                    .size(SIZE_TEXT),
            )
            .push(
                Text::new(product.price.to_bigdecimal(TO_DECIMAL_DIGITS).to_string())
                    .width(Length::FillPortion(2))
                    .size(SIZE_TEXT),
            )
    }

    pub fn scan_barcodes_view(&mut self) -> Element<AppEvents> {
        let mut general_container = Column::new()
            .padding(20)
            .spacing(SPACE_COLUMNS)
            .align_items(iced::Alignment::Center);

        let mut products_container = Column::new()
            .spacing(SPACE_COLUMNS)
            .push(Self::get_list_products_header());

        let are_products: bool = !self.sale_info.products.is_empty();

        self.sale_info.total_pay = PgMoney(0);
        for (key, product) in self.sale_info.products.iter_mut() {
            self.sale_info.total_pay += product.price;
            products_container = products_container.push(
                Self::format_product_row(product).push(
                    Button::new(
                        &mut product.delete_btn_state,
                        Text::new('\u{F1F8}'.to_string()).font(fonts::ICONS),
                    )
                    .on_press(AppEvents::SaleRemoveProductToBuyList(key.to_string())),
                ),
            );
        }

        let products_container = Scrollable::new(&mut self.scroll_list_state)
            .push(products_container)
            .width(Length::Fill)
            .height(Length::Fill);

        general_container = general_container.push(products_container).push(
            Text::new(format!(
                "Total: {}",
                self.sale_info.total_pay.to_bigdecimal(TO_DECIMAL_DIGITS)
            ))
            .size(SIZE_TEXT),
        );

        if are_products {
            general_container = general_container.push(
                Row::new()
                    .spacing(10)
                    .push(
                        Button::new(&mut self.cancel_list_to_pay_state, Text::new("Cancelar"))
                            .on_press(AppEvents::SaleProductsToBuyCancel),
                    )
                    .push(
                        Button::new(&mut self.ok_list_to_pay_state, Text::new("Ok"))
                            .on_press(AppEvents::SaleProductsToBuyOk),
                    ),
            );
        }

        general_container.into()
    }
}
