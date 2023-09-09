//! Icons to be used in the app

use iced::widget::{text, Text};
use iced_aw::{Icon, ICON_FONT};

fn icon(unicode: char) -> Text<'static> {
    text(unicode).font(ICON_FONT)
}

pub fn cancel_icon() -> Text<'static> {
    icon(Icon::XSquare.into())
}

pub fn trash_icon() -> Text<'static> {
    icon(Icon::TrashFill.into())
}

pub fn catalog_icon() -> Text<'static> {
    icon(Icon::ClipboardData.into())
}

pub fn sale_icon() -> Text<'static> {
    icon(Icon::Cart3.into())
}

pub fn loan_icon() -> Text<'static> {
    icon(Icon::Receipt.into())
}

pub fn sale_info_icon() -> Text<'static> {
    icon(Icon::Activity.into())
}

pub fn tobuy_icon() -> Text<'static> {
    icon(Icon::Bag.into())
}

pub fn sale_car_icon() -> Text<'static> {
    icon(Icon::Cart4.into())
}

pub fn check_icon() -> Text<'static> {
    icon(Icon::CheckCircleFill.into())
}

pub fn pencil_icon() -> Text<'static> {
    icon(Icon::Pencil.into())
}

pub fn plus_icon() -> Text<'static> {
    icon(Icon::PlusCircleFill.into())
}

pub fn child_icon() -> Text<'static> {
    icon(Icon::ListNested.into())
}
