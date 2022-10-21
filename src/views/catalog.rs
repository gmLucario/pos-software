use iced::{
    pick_list, Alignment, Button, Column, Element, Length, Row, Scrollable, Text, TextInput,
};

use crate::{
    constants::{SIZE_BTNS_TEXT, SPACE_COLUMNS},
    constants::{SIZE_TEXT, SIZE_TEXT_INPUT, SIZE_TEXT_LABEL},
    kinds::{AppEvents, CatalogInputs, UnitsMeasurement},
};

use crate::controllers::catalog::Catalog;

impl Catalog {
    pub fn populate_record_view(&mut self) -> Element<AppEvents> {
        Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(60)
            .spacing(10)
            .align_items(Alignment::Start)
            .push(
                Text::new(format!("Código Barras:  {}", self.load_product.barcode)).size(SIZE_TEXT),
            )
            .push(Text::new("Producto:").size(SIZE_TEXT_LABEL))
            .push(
                TextInput::new(
                    &mut self.full_name_input_state,
                    "",
                    &self.load_product.product_name,
                    |input_value| {
                        AppEvents::CatalogInputChanged(input_value, CatalogInputs::ProductName)
                    },
                )
                .size(SIZE_TEXT_INPUT),
            )
            .push(Text::new("Cantidad:").size(SIZE_TEXT_LABEL))
            .push(
                Row::new()
                    .push(
                        TextInput::new(
                            &mut self.amount_input_state,
                            "",
                            &self.load_product.amount,
                            |input_value| {
                                AppEvents::CatalogInputChanged(
                                    input_value,
                                    CatalogInputs::AmountProduct,
                                )
                            },
                        )
                        .size(SIZE_TEXT_INPUT),
                    )
                    .push(pick_list::PickList::new(
                        &mut self.pick_list_state,
                        &UnitsMeasurement::ALL[..],
                        Some(self.load_product.unit_measurement),
                        AppEvents::CatalogPickListSelected,
                    )),
            )
            .push(Text::new("Cantidad Mínima:").size(SIZE_TEXT_LABEL))
            .push(
                TextInput::new(
                    &mut self.min_amount_input_state,
                    "",
                    &self.load_product.min_amount,
                    |input_value| {
                        AppEvents::CatalogInputChanged(input_value, CatalogInputs::MinAmountProduct)
                    },
                )
                .size(SIZE_TEXT_INPUT),
            )
            .push(Text::new("Precio Cliente:").size(SIZE_TEXT_LABEL))
            .push(
                TextInput::new(
                    &mut self.user_price_input_state,
                    "",
                    &self.load_product.user_price,
                    |input_value| {
                        AppEvents::CatalogInputChanged(input_value, CatalogInputs::ClientPrice)
                    },
                )
                .size(SIZE_TEXT_INPUT),
            )
            .push(Text::new("Costo:").size(SIZE_TEXT_LABEL))
            .push(
                TextInput::new(
                    &mut self.cost_input_state,
                    "",
                    &self.load_product.cost,
                    |input_value| {
                        AppEvents::CatalogInputChanged(input_value, CatalogInputs::CostProduct)
                    },
                )
                .size(SIZE_TEXT_INPUT),
            )
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.cancel_record_state, Text::new("Cancelar"))
                            .on_press(AppEvents::CatalogNewRecordCancel),
                    )
                    .push(
                        Button::new(&mut self.save_record_state, Text::new("Ok"))
                            .on_press(AppEvents::CatalogNewRecordOk),
                    )
                    .spacing(20),
            )
            .into()
    }

    pub fn catalog_list_view(&mut self) -> Element<AppEvents> {
        let mut container_products = Column::new()
            .spacing(SPACE_COLUMNS)
            .align_items(Alignment::Start);

        let is_products_empty: bool = self.products_to_add.is_empty();

        for (id, product) in self.products_to_add.iter_mut() {
            container_products = container_products
                .push::<Element<AppEvents>>(product.get_formatted_row(id.to_string()))
        }

        let container_products = Scrollable::new(&mut self.scroll_list_state)
            .push(container_products)
            .width(Length::Fill)
            .height(Length::Fill);

        let mut general_container = Column::new()
            .padding(20)
            .spacing(SPACE_COLUMNS)
            .align_items(Alignment::Center)
            .push(
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
                        Text::new("Costo:")
                            .width(Length::FillPortion(2))
                            .size(SIZE_TEXT),
                    )
                    .push(Text::new("").size(SIZE_TEXT)),
            )
            .push(container_products);

        let mut btn_save = Button::new(
            &mut self.save_list_records_state,
            Text::new("Guardar").size(SIZE_BTNS_TEXT),
        );

        if !is_products_empty {
            btn_save = btn_save.on_press(AppEvents::CatalogSaveAllRecords)
        }

        general_container = general_container.push(btn_save);
        general_container.into()
    }
}
