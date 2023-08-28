//! Icons to be used in the app

use iced::widget::{text, Text};
use iced_aw::{Icon, ICON_FONT};

fn icon(unicode: char) -> Text<'static> {
    text(unicode).font(ICON_FONT)
}

pub fn cancel_icon() -> Text<'static> {
    icon(Icon::X.into())
}

pub fn trash_icon() -> Text<'static> {
    icon(Icon::Trash.into())
}

pub fn check_icon() -> Text<'static> {
    icon(Icon::Check.into())
}

pub fn pencil_icon() -> Text<'static> {
    icon(Icon::Pencil.into())
}

pub fn plus_icon() -> Text<'static> {
    icon(Icon::Plus.into())
}

pub fn arrow_left_icon() -> Text<'static> {
    icon(Icon::ArrowLeft.into())
}
