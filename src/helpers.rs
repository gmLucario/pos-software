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
    button(icon::trash_icon()).style(get_style_btn_danger())
}

/// Button styled with `Ok` label
pub fn get_btn_ok<'a>() -> Button<'a, AppEvent> {
    button(icon::check_icon()).style(get_style_btn_ok())
}

/// Button styled with `..` label to edit row
pub fn get_btn_edit<'a>() -> Button<'a, AppEvent> {
    button(icon::pencil_icon())
}

/// Button styled with `Cancelar` label
pub fn get_btn_cancel<'a>() -> Button<'a, AppEvent> {
    button(icon::cancel_icon()).style(get_style_btn_danger())
}

/// Button styled with `Guardar` label
pub fn get_btn_save<'a>() -> Button<'a, AppEvent> {
    button(text("Guardar").size(SIZE_BTNS_TEXT)).style(get_style_btn_ok())
}

/// Button styled with `+` icon
pub fn get_btn_plus_icon<'a>() -> Button<'a, AppEvent> {
    button(icon::plus_icon().size(SIZE_TEXT)).style(get_style_btn_ok())
}

/// Button styled with `menu` icon
pub fn get_btn_right_arrow<'a>() -> Button<'a, AppEvent> {
    button(icon::arrow_left_icon().size(SIZE_TEXT)).style(get_style_btn_ok())
}

/// Validate `input_value` based on `unit_measurement`
/// `UnitsMeasurement::Kilograms | UnitsMeasurement::Liters`: float
/// `UnitsMeasurement::Pieces`: integer
pub fn is_valid_input_text_value_for_amount_data(
    input_value: &str,
    unit_measurement: &UnitsMeasurement,
) -> bool {
    match unit_measurement {
        UnitsMeasurement::Kilograms | UnitsMeasurement::Liters => {
            input_value.parse::<f64>().unwrap_or(-1.0) >= 0.0
        }
        UnitsMeasurement::Pieces if input_value.parse::<u64>().is_ok() => true,
        _ => false,
    }
}

/// Validate if a str is a valid money amount
pub fn is_amount_money_valid(amount: &str) -> bool {
    amount.parse::<f64>().unwrap_or(-1.0) >= 0.0
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
