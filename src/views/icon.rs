//! Icons to be used in the app

use iced::font;

/// Icon's font to be used in the app
pub const FONT_ICONS: font::Font = font::Font::with_name("icons");

#[derive(Clone, Copy, Debug)]
/// An enumeration of all available icons in the [`crate::views::icon::FONT_ICONS`].
/// See: <https://fontello.com/>
pub enum Icon {
    /// Trash
    Trash,
    /// Check
    Check,
    /// Cancel
    Cancel,
    /// Pencil
    Pencil,
    /// Plus
    Plus,
    /// Right arrow item
    ArrowItem,
}

pub const fn icon_to_char(icon: Icon) -> char {
    match icon {
        Icon::Trash => '\u{E800}',
        Icon::Check => '\u{E801}',
        Icon::Cancel => '\u{E802}',
        Icon::Pencil => '\u{E803}',
        Icon::Plus => '\u{E804}',
        Icon::ArrowItem => '\u{E805}',
    }
}

impl From<Icon> for char {
    #[cfg_attr(coverage, no_coverage)]
    fn from(icon: Icon) -> Self {
        icon_to_char(icon)
    }
}

impl From<Icon> for String {
    #[cfg_attr(coverage, no_coverage)]
    fn from(icon: Icon) -> Self {
        icon_to_char(icon).into()
    }
}

impl std::fmt::Display for Icon {
    #[cfg_attr(coverage, no_coverage)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", icon_to_char(*self))
    }
}
