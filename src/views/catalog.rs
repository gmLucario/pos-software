//! Create [`iced::Element`]s to be shown in sale module

use std::collections::HashMap;

use iced::{
    widget::{column, pick_list, row, scrollable, text, text_input},
    Alignment, Element, Length,
};

use crate::{
    constants::{COLUMN_PADDING, SIZE_TEXT, SIZE_TEXT_INPUT, SIZE_TEXT_LABEL, SPACE_COLUMNS},
    helpers::{get_btn_cancel, get_btn_ok, get_btn_save, get_btn_trash_icon},
    kinds::{AppEvents, CatalogInputs, UnitsMeasurement},
    schemas::catalog::LoadProduct,
};

/// Get a new product row for the catalog list
fn catalog_list_view_formatted_row(id: String, product: &LoadProduct) -> Element<AppEvents> {
    row!(
        text(format!("{}: {}", product.barcode, product.product_name))
            .size(SIZE_TEXT)
            .width(Length::FillPortion(6)),
        text(&product.amount)
            .size(SIZE_TEXT)
            .width(Length::FillPortion(2)),
        text(&product.cost)
            .size(SIZE_TEXT)
            .width(Length::FillPortion(2)),
        get_btn_trash_icon().on_press(AppEvents::CatalogRemoveRecordList(id)),
    )
    .spacing(10)
    .width(iced::Length::Fill)
    .into()
}

/// Defines the form which the user will define the
/// product info to be added in the catalog
pub fn load_product_view(product: &LoadProduct) -> Element<AppEvents> {
    column!(
        text(format!("Código Barras:  {}", product.barcode)).size(SIZE_TEXT),
        text("Producto:").size(SIZE_TEXT_LABEL),
        text_input("", &product.product_name, |input_value| {
            AppEvents::CatalogInputChanged(input_value, CatalogInputs::ProductName)
        })
        .size(SIZE_TEXT_INPUT),
        text("Cantidad:").size(SIZE_TEXT_LABEL),
        row!(
            text_input("", &product.amount, |input_value| {
                AppEvents::CatalogInputChanged(input_value, CatalogInputs::AmountProduct)
            })
            .size(SIZE_TEXT_INPUT),
            pick_list(
                &UnitsMeasurement::ALL[..],
                Some(product.unit_measurement),
                AppEvents::CatalogPickListSelected,
            )
        ),
        text("Cantidad Mínima:").size(SIZE_TEXT_LABEL),
        text_input("", &product.min_amount, |input_value| {
            AppEvents::CatalogInputChanged(input_value, CatalogInputs::MinAmountProduct)
        })
        .size(SIZE_TEXT_INPUT),
        text("Precio Cliente:").size(SIZE_TEXT_LABEL),
        text_input("", &product.user_price, |input_value| {
            AppEvents::CatalogInputChanged(input_value, CatalogInputs::ClientPrice)
        })
        .size(SIZE_TEXT_INPUT),
        text("Costo:").size(SIZE_TEXT_LABEL),
        text_input("", &product.cost, |input_value| {
            AppEvents::CatalogInputChanged(input_value, CatalogInputs::CostProduct)
        })
        .size(SIZE_TEXT_INPUT),
        row!(
            get_btn_cancel().on_press(AppEvents::CatalogNewRecordCancel),
            get_btn_ok().on_press(AppEvents::CatalogNewRecordOk),
        )
        .spacing(20),
    )
    .height(Length::Fill)
    .width(Length::Fill)
    .padding(60)
    .spacing(SPACE_COLUMNS)
    .align_items(Alignment::Start)
    .into()
}

/// Defines list products to be added in the catalog
pub fn catalog_list_view(products: &HashMap<String, LoadProduct>) -> Element<AppEvents> {
    let mut container_products = column!()
        .spacing(SPACE_COLUMNS)
        .align_items(Alignment::Start);

    let is_products_empty: bool = products.is_empty();

    for (id, product) in products {
        container_products =
            container_products.push(catalog_list_view_formatted_row(id.to_string(), product))
    }

    let container_products = scrollable(container_products).height(Length::Fill);

    let mut general_container = column!(
        row!(
            text("Producto:")
                .width(Length::FillPortion(5))
                .size(SIZE_TEXT),
            text("Cantidad:")
                .width(Length::FillPortion(2))
                .size(SIZE_TEXT),
            text("Costo:").width(Length::FillPortion(2)).size(SIZE_TEXT),
            text("").size(SIZE_TEXT)
        ),
        container_products,
    )
    .padding(COLUMN_PADDING)
    .spacing(SPACE_COLUMNS)
    .align_items(Alignment::Center);

    let mut btn_save = get_btn_save();

    if !is_products_empty {
        btn_save = btn_save.on_press(AppEvents::CatalogSaveAllRecords)
    }

    general_container = general_container.push(btn_save);
    general_container.into()
}
