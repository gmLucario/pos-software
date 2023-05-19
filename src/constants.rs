//! Constant app values

use iced::Color;

// Labels
/// Text shows in the tittle bar of the main window app
pub const WINDOW_TITTLE: &str = "Pos Software";
/// Text shows when the stock is empty
pub const STOCK_IS_EMPTY_MSG: &str = "Su catálogo esta vacío";
/// Text shows when the catalog form is not fully filled
pub const ASK_FILL_CATALOG_FORM: &str = "Llenar todos los campos del formulario";
pub const NO_PRODUCT: &str = "Producto no existente";

// Buttons labels
/// Catalog button label
pub const CATALOG_BTN_MSG: &str = "Catálogo";
/// Sale button label
pub const SALE_BTN_MSG: &str = "Venta";
/// Sales info button label
pub const SALES_INFO_BTN_MSG: &str = "Estadísticas";
/// Product to buy button label
pub const TO_BUY_BTN_MSG: &str = "Lista Compra";
/// Loans button label
pub const LOAN_BTN_MSG: &str = "Prestamos";
/// General error message to retry the action
pub const GENERAL_RETRY_MSG: &str = "Error, favor de intentar de nuevo";
/// General success message
pub const GENERAL_SUCCESS_MSG: &str = "Proceso éxitoso";

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
/// Horizontal space between each element of a row container
pub const SPACE_ROW_BTNS_FORM: u16 = 20;
/// Vertical space between each element of a column container
pub const SPACE_COLUMNS: u16 = 10;
/// Padding for column items
pub const COLUMN_PADDING: u16 = 20;
/// Forms padding
pub const FORM_PADDING: u16 = 30;
/// Max width size for [`custom_crates::widgets::modal::Modal`]
pub const MODAL_MAX_WIDTH: u32 = 500;
/// Max height size for [`custom_crates::widgets::modal::Modal`]
pub const MODAL_MAX_HEIGHT: u32 = 300;
/// Pading value for modal's content
pub const MODAL_PADING_CONTENT: u16 = 300;

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
    r: 0.14118,
    g: 0.12157,
    b: 0.19216,
    a: 1.0,
};

/// Color for list btns
///
/// Use [color picker](https://ajalt.github.io/colormath/converter/) as references
pub const COLUMN_LIST_BTNS: Color = Color {
    r: 0.592,
    g: 0.251,
    b: 0.749,
    a: 1.0,
};
