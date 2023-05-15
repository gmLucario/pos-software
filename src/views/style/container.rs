
//! Define application containers styles
use iced::{theme, widget::container, Color, Theme};

/// Represents types containers of the application
#[derive(Default)]
enum Container {
    /// Container with black border
    #[default]
    BorderBlack,
}

impl container::StyleSheet for Container {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        match self {
            Container::BorderBlack => container::Appearance {
                border_radius: 1.0,
                border_width: 0.5,
                border_color: Color::BLACK,
                ..container::Appearance::default()
            },
        }
    }
}

pub fn get_black_border_style() -> theme::Container {
    theme::Container::Custom(Box::from(Container::BorderBlack))
}
