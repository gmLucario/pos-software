use iced::{widget::button::StyleSheet, Color};

use crate::constants::{DEFAULT_BLUE, DEFAULT_GREEN, DEFAULT_RED};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    #[default]
    Ok,
    Cancel,
    Primary,
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
