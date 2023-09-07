//! Icons to be used in the app

use iced::widget::{text, Text};
use iced_aw::ICON_FONT;

pub const ICON_FONT_BYTES: &[u8] = include_bytes!("../../assets/bootstrap-icons.ttf");

fn icon(unicode: char) -> Text<'static> {
    text(unicode).font(ICON_FONT)
}

pub fn cancel_icon() -> Text<'static> {
    icon('\u{F629}')
}

pub fn trash_icon() -> Text<'static> {
    icon('\u{F5DD}')
}

pub fn catalog_icon() -> Text<'static> {
    icon('\u{F28C}')
}

pub fn sale_icon() -> Text<'static> {
    icon('\u{F244}')
}

pub fn loan_icon() -> Text<'static> {
    icon('\u{F50F}')
}

pub fn sale_info_icon() -> Text<'static> {
    icon('\u{F66B}')
}

pub fn tobuy_icon() -> Text<'static> {
    icon('\u{F179}')
}

pub fn sale_car_icon() -> Text<'static> {
    icon('\u{F245}')
}

pub fn check_icon() -> Text<'static> {
    icon('\u{F26A}')
}

pub fn pencil_icon() -> Text<'static> {
    icon('\u{F4CB}')
}

pub fn plus_icon() -> Text<'static> {
    icon('\u{F4F9}')
}

pub fn child_icon() -> Text<'static> {
    icon('\u{F474}')
}
