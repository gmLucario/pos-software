//! User interfaces shown in catalog module

use std::collections::BTreeMap;

use iced::{
    widget::{column, container, pick_list, row, scrollable, text, text_input, Column},
    Alignment, Element, Length,
};

use crate::{
    constants::{
        COLUMN_PADDING, FORM_PADDING, SIZE_TEXT, SIZE_TEXT_INPUT, SIZE_TEXT_LABEL, SPACE_COLUMNS,
        SPACE_ROWS, SPACE_ROW_BTNS_FORM,
    },
    events::AppEvent,
    helpers::{get_btn_cancel, get_btn_edit, get_btn_ok, get_btn_save, get_btn_trash_icon},
    kinds::{OnScroll, PickList, TextInput, UnitsMeasurement, View},
    models::catalog::{LoadProduct, ProductAmount},
    schemas::catalog::CatalogProductForm,
};

/// Show products in the catalog
pub fn show_list_products<'a>(
    products: &[ProductAmount],
    text_input_value: &str,
) -> Element<'a, AppEvent> {
    let products: Vec<Element<AppEvent>> = products
        .iter()
        .map(|pr| text(pr.get_formatted_item()).size(SIZE_TEXT).into())
        .collect();

    let col = Column::with_children(products)
        .spacing(SPACE_COLUMNS)
        .width(Length::Fill);

    column!(
        text_input("", text_input_value)
            .on_input(|input_value| {
                AppEvent::TextInputChanged(input_value, TextInput::CatalogFilterStockList)
            })
            .on_submit(AppEvent::ChangeView(View::CatalogProducts))
            .size(SIZE_TEXT_INPUT),
        container(
            scrollable(col)
                .on_scroll(|offset| {
                    AppEvent::ScrollScrolled(OnScroll::CatalogListProducts, offset)
                })
                .height(Length::Fill)
        )
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .padding(COLUMN_PADDING)
    .spacing(SPACE_COLUMNS)
    .into()
}

/// Form to be used to define the product to be added in the catalog
pub fn product_form<'a>(product: &CatalogProductForm, is_edit: bool) -> Element<'a, AppEvent> {
    let form_product = column!(
        text("Código Barras:").size(SIZE_TEXT_LABEL),
        text_input("", &product.barcode)
            .on_input(|input_value| {
                AppEvent::TextInputChanged(input_value, TextInput::CatalogFormBarcode)
            })
            .on_submit(AppEvent::CatalogRequestProductInfoForm)
            .size(SIZE_TEXT_INPUT),
        text("Producto:").size(SIZE_TEXT_LABEL),
        text_input("", &product.product_name)
            .on_input(|input_value| {
                AppEvent::TextInputChanged(input_value, TextInput::CatalogFormProductName)
            })
            .size(SIZE_TEXT_INPUT),
        text("Cantidad:").size(SIZE_TEXT_LABEL),
        row!(
            text_input("", &product.amount)
                .on_input(|input_value| {
                    AppEvent::TextInputChanged(input_value, TextInput::CatalogFormAmountProduct)
                })
                .size(SIZE_TEXT_INPUT),
            pick_list(
                &UnitsMeasurement::ALL[..],
                Some(product.unit_measurement),
                |unit| {
                    AppEvent::PickListSelected(PickList::CatalogFormPickListUnitMeasurement(unit))
                },
            )
        ),
        text("Cantidad Mínima:").size(SIZE_TEXT_LABEL),
        text_input("", &product.min_amount)
            .on_input(|input_value| {
                AppEvent::TextInputChanged(input_value, TextInput::CatalogFormMinAmountProduct)
            })
            .size(SIZE_TEXT_INPUT),
        text("Precio Cliente:").size(SIZE_TEXT_LABEL),
        text_input("", &product.user_price)
            .on_input(|input_value| {
                AppEvent::TextInputChanged(input_value, TextInput::CatalogFormClientPrice)
            })
            .size(SIZE_TEXT_INPUT),
        text("Costo:").size(SIZE_TEXT_LABEL),
        text_input("", &product.cost)
            .on_input(|input_value| {
                AppEvent::TextInputChanged(input_value, TextInput::CatalogFormCostProduct)
            })
            .size(SIZE_TEXT_INPUT),
        row!(
            get_btn_cancel().on_press(AppEvent::CatalogNewRecordListTobeSavedCancel),
            get_btn_ok().on_press(AppEvent::CatalogNewRecordListTobeSavedOk(is_edit)),
        )
        .spacing(SPACE_ROW_BTNS_FORM),
    )
    .height(Length::Fill)
    .width(Length::Fill)
    .padding(FORM_PADDING)
    .spacing(SPACE_COLUMNS);

    form_product.into()
}

/// Products to be added into the catalog
pub fn products_be_added_catalog(
    products: &BTreeMap<std::string::String, LoadProduct>,
) -> Element<AppEvent> {
    column![
        scrollable(
            column(
                products
                    .iter()
                    .map(|(_, product)| products_be_added_catalog_row(product))
                    .collect(),
            )
            .spacing(SPACE_COLUMNS)
        )
        .height(Length::Fill),
        get_btn_save().on_press(AppEvent::CatalogSaveAllRecords)
    ]
    .align_items(Alignment::Center)
    .spacing(SPACE_COLUMNS)
    .padding(COLUMN_PADDING)
    .into()
}

fn products_be_added_catalog_row(product: &LoadProduct) -> Element<AppEvent> {
    let label = format!(
        "- {amount}[{unit_measurement}] {product_name}",
        product_name = product.product_name,
        amount = product.current_amount,
        unit_measurement = UnitsMeasurement::from(product.unit_measurement_id),
    );
    let label = text(label).size(SIZE_TEXT).width(Length::Fill);

    row!(
        label,
        get_btn_edit().on_press(AppEvent::CatalogEditRecordListTobeSaved(
            product.barcode.to_string()
        )),
        get_btn_trash_icon().on_press(AppEvent::CatalogRemoveRecordListTobeSaved(
            product.barcode.to_string()
        )),
    )
    .spacing(SPACE_ROWS)
    .into()
}
