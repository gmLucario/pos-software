//! Define application button styles

use iced::{widget::button::StyleSheet, Color};

use crate::constants::{DEFAULT_DEACTIVATE, DEFAULT_GREEN, DEFAULT_MENU, DEFAULT_RED};

/// Represents types buttons of the application
#[derive(Default)]
enum Button {
    /// Button user agree
    #[default]
    Ok,
    /// Button user is not agree or delete
    Danger,
    /// Main menu nav bar items
    MainMenu,
}

impl StyleSheet for Button {
    type Style = iced::Theme;

    /// Produces the disabled [`iced::widget::button::Appearance`] of a button.
    fn active(&self, _: &Self::Style) -> iced::widget::button::Appearance {
        let basic_appearance = |color: Color| iced::widget::button::Appearance {
            text_color: color,
            border_color: color,
            border_width: 1.0,
            border_radius: 2.0,
            ..iced::widget::button::Appearance::default()
        };

        match self {
            Button::Ok => basic_appearance(DEFAULT_GREEN),
            Button::Danger => basic_appearance(DEFAULT_RED),
            Button::MainMenu => basic_appearance(DEFAULT_MENU),
        }
    }

    /// Produces the disabled [`iced::widget::button::Appearance`] of a button.
    fn disabled(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let active = self.active(style);

        iced::widget::button::Appearance {
            text_color: DEFAULT_DEACTIVATE,
            border_color: DEFAULT_DEACTIVATE,
            ..active
        }
    }
}

/// Return the style for buttons `danger`
pub fn get_style_btn_danger() -> iced::theme::Button {
    iced::theme::Button::Custom(Box::from(crate::style::btns::Button::Danger))
}

/// Return the style for buttons `ok`
pub fn get_style_btn_ok() -> iced::theme::Button {
    iced::theme::Button::Custom(Box::from(crate::style::btns::Button::Ok))
}

/// Return the style for buttons `main menu`
pub fn get_style_btn_main_menu() -> iced::theme::Button {
    iced::theme::Button::Custom(Box::from(crate::style::btns::Button::MainMenu))
}
