//! General fn helpers

use custom_crates::widgets::toast;
use iced::{
    widget::{button, text, Button},
    Command,
};

use crate::{
    constants::{SIZE_BTNS_TEXT, SIZE_TEXT},
    events::AppEvent,
    kinds::UnitsMeasurement,
    views::{
        icon,
        style::btns::{get_style_btn_danger, get_style_btn_ok},
    },
};

/// Button styled with a trash icon
pub fn get_btn_trash_icon<'a>() -> Button<'a, AppEvent> {
    button(text(icon::Icon::Trash).font(icon::FONT_ICONS)).style(get_style_btn_danger())
}

/// Button styled with `Ok` label
pub fn get_btn_ok<'a>() -> Button<'a, AppEvent> {
    button(text(icon::Icon::Check).font(icon::FONT_ICONS)).style(get_style_btn_ok())
}

/// Button styled with `..` label to edit row
pub fn get_btn_edit<'a>() -> Button<'a, AppEvent> {
    button(text(icon::Icon::Pencil).font(icon::FONT_ICONS))
}

/// Button styled with `Cancelar` label
pub fn get_btn_cancel<'a>() -> Button<'a, AppEvent> {
    button(text(icon::Icon::Cancel).font(icon::FONT_ICONS)).style(get_style_btn_danger())
}

/// Button styled with `Guardar` label
pub fn get_btn_save<'a>() -> Button<'a, AppEvent> {
    button(text("Guardar").size(SIZE_BTNS_TEXT)).style(get_style_btn_ok())
}

/// Button styled with `+` icon
pub fn get_btn_plus_icon<'a>() -> Button<'a, AppEvent> {
    button(
        text(icon::Icon::Plus)
            .font(icon::FONT_ICONS)
            .size(SIZE_TEXT),
    )
    .style(get_style_btn_ok())
}

/// Validate `input_value` based on `unit_measurement`
/// `UnitsMeasurement::Kilograms | UnitsMeasurement::Liters`: float
/// `UnitsMeasurement::Pieces`: integer
pub fn is_valid_input_text_value_for_amount_data(
    input_value: &str,
    unit_measurement: &UnitsMeasurement,
) -> bool {
    match unit_measurement {
        UnitsMeasurement::Kilograms | UnitsMeasurement::Liters
            if input_value.parse::<f64>().is_ok() =>
        {
            true
        }
        UnitsMeasurement::Pieces if input_value.parse::<u64>().is_ok() => true,
        _ => false,
    }
}

/// Send an err toast type to the app
pub fn send_toast_err(msg: String) -> Command<AppEvent> {
    Command::perform(async {}, |_| AppEvent::AddToast(toast::Status::Danger, msg))
}
/// Send an ok toast type to the app
pub fn send_toast_ok(msg: String) -> Command<AppEvent> {
    Command::perform(async {}, |_| {
        AppEvent::AddToast(toast::Status::Primary, msg)
    })
}
