//! Commonly used types used across the crate

use std::fmt::Display;

use crate::models::{
    loan::LoanPayment,
    sale::{ProductSale, SaleProductInfo},
};

/// Available app modules
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AppModule {
    /// Products to availables to be saled
    Catalog,
    /// Perform a sale flow
    #[default]
    Sale,
    /// Handle loans
    Loans,
    /// Show the list of filtered products to be bought
    ToBuyList,
    /// Main info about the sales, products, loans
    Stats,
}

/// All the possible app views
#[derive(Debug, Clone, Default, PartialEq)]
pub enum View {
    /// List of products in the catalago filtered
    CatalogProducts,
    /// List of products to be added/created into the catalog
    CatalogProductsToBeAdded,
    /// Form to add a new record to the list of products
    CatalogAddProductForm(bool),

    /// Sale view with the products to be te by the client
    #[default]
    SaleListProducts,

    /// List loans filter by debtor name
    LoansByDeptor,

    /// List products to be bought to fill the stock
    ToBuy,

    /// Statistics about sales
    SaleInfo,
}

/// Available modal views
#[derive(Debug, Clone)]
pub enum ModalView {
    SaleProductAddEditForm(SaleProductInfo, bool),
    LoanSaleDetails(Vec<ProductSale>),
    LoanPayments(Vec<LoanPayment>),
}

/// Available picklists
#[derive(Debug, Clone, PartialEq)]
pub enum PickList {
    CatalogFormPickListUnitMeasurement(UnitsMeasurement),
}

/// Available textinputs
#[derive(Debug, Clone, Copy)]
pub enum TextInput {
    CatalogFilterStockList,
    CatalogFormBarcode,
    CatalogFormProductName,
    CatalogFormAmountProduct,
    CatalogFormMinAmountProduct,
    CatalogFormClientPrice,
    CatalogFormCostProduct,

    SaleFormProductAmount(UnitsMeasurement),
    SaleUserPayment,
    SaleClientNameLoan,

    LoanDebtorName,
    LoanPaymentAmountLoan,

    /// Product like value to filter the product to be bought
    ToBuyProductLike,
}

/// Types of valid units measurement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnitsMeasurement {
    Kilograms, // 1
    Liters,    // 2
    #[default]
    Pieces, // 3
}

/// Types of date picker in the hole app
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppDatePicker {
    /// Date search starts
    SaleStartDatePicker,
    /// Date search ends
    SaleEndDatePicker,
}

impl Default for ModalView {
    fn default() -> ModalView {
        ModalView::SaleProductAddEditForm(SaleProductInfo::default(), false)
    }
}

/// Converts from `i16` to [crate::kinds::UnitsMeasurement] enum
impl From<i16> for UnitsMeasurement {
    fn from(value: i16) -> Self {
        match value {
            1 => UnitsMeasurement::Kilograms,
            2 => UnitsMeasurement::Liters,
            3 => UnitsMeasurement::Pieces,
            _ => UnitsMeasurement::Pieces,
        }
    }
}

/// Converts from [crate::kinds::UnitsMeasurement] enum to `i16`
impl From<UnitsMeasurement> for i16 {
    fn from(unit: UnitsMeasurement) -> Self {
        match unit {
            UnitsMeasurement::Kilograms => 1,
            UnitsMeasurement::Liters => 2,
            UnitsMeasurement::Pieces => 3,
        }
    }
}

/// User-facing output for [crate::kinds::UnitsMeasurement]
impl Display for UnitsMeasurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            UnitsMeasurement::Kilograms => "Kilogramos",
            UnitsMeasurement::Liters => "Litros",
            UnitsMeasurement::Pieces => "Piezas",
        };
        write!(f, "{msg}")
    }
}

impl UnitsMeasurement {
    /// List of all the valid units measurement of the app
    pub const ALL: [UnitsMeasurement; 3] = [
        UnitsMeasurement::Kilograms,
        UnitsMeasurement::Liters,
        UnitsMeasurement::Pieces,
    ];
}
