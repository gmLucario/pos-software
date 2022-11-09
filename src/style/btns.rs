//! Define application button styles

use iced::{widget::button::StyleSheet, Color};

use crate::constants::{DEFAULT_BLUE, DEFAULT_GREEN, DEFAULT_RED};

/// Represents types buttons of the application
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    /// Button user agree
    Ok,
    /// Button user is not agree
    Cancel,
    /// General purpose button
    #[default]
    Primary,
    /// Main menu nav bar items
    MainMenu,
}

impl StyleSheet for Button {
    fn active(&self) -> iced::button::Style {
        let basic_appearance = |color: Color| iced::button::Style {
            text_color: color,
            border_color: color,
            border_width: 1.0,
            border_radius: 2.0,
            ..iced::button::Style::default()
        };

        match self {
            Button::Ok => basic_appearance(DEFAULT_GREEN),
            Button::Cancel => basic_appearance(DEFAULT_RED),
            Button::Primary => basic_appearance(DEFAULT_BLUE),
            Button::MainMenu => basic_appearance(Color::BLACK),
        }
    }
}
