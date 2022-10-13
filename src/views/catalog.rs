use iced::{
    widget::pick_list, Alignment, Button, Column, Element, Length, Row, Scrollable, Text, TextInput,
};

use crate::constants::SPACE_COLUMNS;
use crate::constants::{SIZE_TEXT, SIZE_TEXT_INPUT, SIZE_TEXT_LABEL};
use crate::kinds::{AppEvents, CatalogInputs, UnitsMeasurement};

use crate::controllers::catalog::Catalog;

impl Catalog {
    pub fn get_fields_form_populate_record(&mut self) -> Vec<Row<AppEvents>> {
        let format_label = |label: &str| -> String { format!("{:<20}", label) };
        let items = vec![
            Row::new()
                .push(
                    Text::new(format!("Código Barras:  {}", self.load_product.barcode))
                        .size(SIZE_TEXT),
                )
                .align_items(Alignment::Center),
            Row::new()
                .push(Text::new(format_label("Producto")).size(SIZE_TEXT_LABEL))
                .push(
                    TextInput::new(
                        &mut self.full_name_input_state,
                        "",
                        &self.load_product.product_name,
                        |input_value| {
                            AppEvents::InputChangedCatalog(input_value, CatalogInputs::ProductName)
                        },
                    )
                    .size(SIZE_TEXT_INPUT),
                ),
            Row::new()
                .push(Text::new(format_label("Cantidad")).size(SIZE_TEXT_LABEL))
                .push(
                    TextInput::new(
                        &mut self.amount_input_state,
                        "",
                        &self.load_product.amount,
                        |input_value| {
                            AppEvents::InputChangedCatalog(
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
            Row::new()
                .push(Text::new(format_label("Cantidad Mínima")).size(SIZE_TEXT_LABEL))
                .push(
                    TextInput::new(
                        &mut self.min_amount_input_state,
                        "",
                        &self.load_product.min_amount,
                        |input_value| {
                            AppEvents::InputChangedCatalog(
                                input_value,
                                CatalogInputs::AmountProduct,
                            )
                        },
                    )
                    .size(SIZE_TEXT_INPUT),
                ),
            Row::new()
                .push(Text::new(format_label("Precio Cliente")).size(SIZE_TEXT_LABEL))
                .push(
                    TextInput::new(
                        &mut self.user_price_input_state,
                        "",
                        &self.load_product.user_price,
                        |input_value| {
                            AppEvents::InputChangedCatalog(input_value, CatalogInputs::ClientPrice)
                        },
                    )
                    .size(SIZE_TEXT_INPUT),
                ),
            Row::new()
                .push(
                    Button::new(&mut self.cancel_record_state, Text::new("Cancelar"))
                        .on_press(AppEvents::CatalogNewRecordCancel),
                )
                .push(
                    Button::new(&mut self.save_record_state, Text::new("Ok"))
                        .on_press(AppEvents::CatalogNewRecordOk),
                )
                .align_items(Alignment::Center)
                .spacing(20),
        ];

        items
    }

    pub fn populate_record_view(&mut self) -> Element<AppEvents> {
        let mut container = Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(60)
            .spacing(10)
            .align_items(Alignment::Center);
        for item in self.get_fields_form_populate_record() {
            container = container.push(item);
        }

        container.into()
    }

    pub fn catalog_list_view(&mut self) -> Element<AppEvents> {
        let container_products = Column::with_children(
            self.products_to_add
                .iter()
                .map(Text::new)
                .map(Element::from)
                .collect(),
        )
        .spacing(SPACE_COLUMNS)
        .align_items(Alignment::Start);

        let container_products = Scrollable::new(&mut self.scroll_list_state)
            .push(container_products)
            .width(Length::Fill)
            .height(Length::Fill);

        Column::new()
            .padding(20)
            .spacing(SPACE_COLUMNS)
            .align_items(Alignment::Start)
            .push(container_products)
            .into()
    }
}
