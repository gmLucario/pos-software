//! Define application button styles

use iced::{theme, widget::button, BorderRadius, Color};

use crate::constants::{COLUMN_LIST_BTNS, DEFAULT_DEACTIVATE, DEFAULT_GREEN};

/// Represents types buttons of the application
#[derive(Default)]
enum Button {
    /// Button user agree
    #[default]
    Ok,
    /// Listed Items
    ListedItems,
}

impl button::StyleSheet for Button {
    type Style = iced::Theme;

    /// Produces the disabled [`iced::widget::button::Appearance`] of a button.
    fn active(&self, _: &Self::Style) -> button::Appearance {
        let basic_appearance = |color: Color| button::Appearance {
            text_color: color,
            border_color: color,
            border_width: 1.0,
            border_radius: BorderRadius::from(2.0),
            ..button::Appearance::default()
        };

        match self {
            Button::Ok => basic_appearance(DEFAULT_GREEN),
            Button::ListedItems => basic_appearance(COLUMN_LIST_BTNS),
        }
    }

    /// Produces the disabled [`iced::widget::button::Appearance`] of a button.
    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            text_color: DEFAULT_DEACTIVATE,
            border_color: DEFAULT_DEACTIVATE,
            ..active
        }
    }
}

/// Return the style for buttons `danger`
pub fn get_style_btn_danger() -> theme::Button {
    theme::Button::Destructive
}

/// Return the style for buttons `ok`
pub fn get_style_btn_ok() -> theme::Button {
    theme::Button::Positive
}

/// Return the style for buttons `main menu`
pub fn get_style_btn_main_menu() -> theme::Button {
    theme::Button::Text
}

/// Return the style for buttons of type `ListedItems`
pub fn get_style_btn_listed_items() -> theme::Button {
    theme::Button::Custom(Box::from(Button::ListedItems))
}
