//! Constant app values

use iced::Color;

// Labels
/// Text shows in the tittle bar of the main window app
pub const WINDOW_TITTLE: &str = "Pos Software";

// Buttons labels
/// Catalog button label
pub const CATALOG_BTN_MSG: &str = "Catálogo";
/// Sale button label
pub const SALE_BTN_MSG: &str = "Venta";
/// Sales info button label
pub const SALES_INFO_BTN_MSG: &str = "Estadísticas";
/// Producto to buy button label
pub const TO_BUY_BTN_MSG: &str = "Lista Compra";

// Sizes
/// Limit of characters saved to be considered as a barcode
/// to avoid the overflow of the variable
pub const CHARS_SAVED_AS_BARCODE: usize = 80;
/// General size for text
pub const SIZE_TEXT: u16 = 30;
/// General size for labels
pub const SIZE_TEXT_LABEL: u16 = 20;
/// General size for input fields texts
pub const SIZE_TEXT_INPUT: u16 = 25;
/// General size for text in buttons
pub const SIZE_BTNS_TEXT: u16 = 30;
/// Horizontal space between each element of a row container
pub const SPACE_ROWS: u16 = 10;
/// Vertical space between each element of a column container
pub const SPACE_COLUMNS: u16 = 10;

// Data
/// Number of decimals from [`sqlx::postgres::types::PgMoney`] to
/// [`sqlx::types::BigDecimal`]
pub const TO_DECIMAL_DIGITS: i64 = 2;
/// Number of decimals from [`sqlx::types::BigDecimal`] to
/// [`sqlx::postgres::types::PgMoney`]
pub const PGMONEY_DECIMALS: u32 = 2;
/// Number of connections allowed in the database pool
pub const MAX_CONNECTIONS_POOL: u32 = 2;

// Colors
/// Default `green` color
///
/// Use [color picker](https://ajalt.github.io/colormath/converter/) as references
pub const DEFAULT_GREEN: Color = Color {
    r: 0.0,
    g: 0.6,
    b: 0.3,
    a: 1.0,
};

/// Default `red` color
///
/// Use [color picker](https://ajalt.github.io/colormath/converter/) as references
pub const DEFAULT_RED: Color = Color {
    r: 0.75,
    g: 0.101,
    b: 0.157,
    a: 1.0,
};

/// Default `deactivate` color
///
/// Use [color picker](https://ajalt.github.io/colormath/converter/) as references
pub const DEFAULT_DEACTIVATE: Color = Color {
    r: 0.7,
    g: 0.7,
    b: 0.7,
    a: 0.5,
};

/// Default `menu` color
///
/// Use [color picker](https://ajalt.github.io/colormath/converter/) as references
pub const DEFAULT_MENU: Color = Color {
    r: 0.250,
    g: 0.750,
    b: 0.7,
    a: 1.0,
};
