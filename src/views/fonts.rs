use iced::Font;

pub const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../../assets/fonts/icons.ttf"),
};
