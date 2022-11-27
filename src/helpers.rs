//! General fn helpers
use iced::widget::{button, text, Button};

use crate::{
    constants::{SIZE_BTNS_TEXT, SIZE_TEXT},
    kinds::AppEvents,
    style::btns::{get_style_btn_danger, get_style_btn_ok},
    views::fonts,
};

/// Button styled with a check icon
pub fn get_btn_check_icon<'a>() -> Button<'a, AppEvents> {
    button(text('\u{f00c}').font(fonts::FONT_ICONS)).style(get_style_btn_ok())
}

/// Button styled with a trash icon
pub fn get_btn_trash_icon<'a>() -> Button<'a, AppEvents> {
    button(text('\u{F1F8}').font(fonts::FONT_ICONS)).style(get_style_btn_danger())
}

/// Button styled with `Ok` label
pub fn get_btn_ok<'a>() -> Button<'a, AppEvents> {
    button(text("Ok")).style(get_style_btn_ok())
}

/// Button styled with `Cancelar` label
pub fn get_btn_cancel<'a>() -> Button<'a, AppEvents> {
    button(text("Cancelar")).style(get_style_btn_danger())
}

/// Button styled with `Guardar` label
pub fn get_btn_save<'a>() -> Button<'a, AppEvents> {
    button(text("Guardar").size(SIZE_BTNS_TEXT)).style(get_style_btn_ok())
}

/// Button styled with `+` icon
pub fn get_btn_plus_icon<'a>() -> Button<'a, AppEvents> {
    button(text("+").size(SIZE_TEXT)).style(get_style_btn_ok())
}
