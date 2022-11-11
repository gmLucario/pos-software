//! Fonts to be used in the app

use iced::Font;

/// Garbage icon to be used in the app
pub const GARBAGE_ICON: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../../assets/fonts/icons.ttf"),
};
