use iced::{Button, Column, Element, Length, Row, Scrollable, Text, TextInput};
use sqlx::postgres::types::PgMoney;

use crate::{
    constants::{SIZE_TEXT, SPACE_COLUMNS, TO_DECIMAL_DIGITS},
    controllers,
    kinds::{AppEvents, SaleInputs},
    models::sale::SaleProductInfo,
};

impl controllers::sale::Sale {
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
                        Button::new(&mut self.cancel_new_record_btn_state, Text::new("Cancel"))
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

    fn format_product_row<'a>(product: &SaleProductInfo) -> Row<'a, AppEvents> {
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
            .push(Text::new("").size(SIZE_TEXT))
    }

    pub fn scan_barcodes_view(&mut self) -> Element<AppEvents> {
        let mut general_container = Column::new()
            .padding(20)
            .spacing(SPACE_COLUMNS)
            .align_items(iced::Alignment::Center);

        let mut products_container = Column::new()
            .spacing(SPACE_COLUMNS)
            .push(Self::get_list_products_header());

        let mut total_pay = PgMoney(0);
        for (_, product) in self.products.iter() {
            total_pay += product.price;
            products_container = products_container.push(Self::format_product_row(product));
        }
        let total_pay = total_pay.to_bigdecimal(TO_DECIMAL_DIGITS);

        let products_container = Scrollable::new(&mut self.scroll_list_state)
            .push(products_container)
            .width(Length::Fill)
            .height(Length::Fill);

        general_container = general_container
            .push(products_container)
            .push(Text::new(format!("Total: {}", total_pay)).size(SIZE_TEXT));

        general_container.into()
    }
}
