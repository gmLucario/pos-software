//! Commonly used types used across the crate

use std::fmt::Display;

use iced_aw::date_picker::Date;
use sqlx::{postgres::types::PgMoney, types::Uuid};

use crate::models::{
    catalog::{ProductInfo, ProductToBuy},
    loan::{LoanItem, LoanPayment, TotalLoans},
    sale::{ProductSale, SaleProductInfo, TotalSales},
};

#[derive(Default)]
/// All the possible app views
pub enum Views {
    #[default]
    /// List products to be sold and total
    Sale,
    /// Form to add a new product to the [`crate::kinds::Views::Sale`] view
    SaleAddProductForm,
    /// Form to input user payment and calculate payback money
    SaleChargeForm,
    /// Statistics about sales
    SalesInfo,
    /// List products to be bought to fill the stock
    ToBuy,
    /// List new products to be added to the catalog
    Catalog,
    /// Form input product details to be added to the catalog
    CatalogAddRecord,
    /// Info about Loans
    LoanInfo,
}

/// Types of valid units measurement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnitsMeasurement {
    Kilograms, // 1
    Liters,    // 2
    #[default]
    Pieces, // 3
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

/// Types of user inputs in [`crate::kinds::Views::Catalog`] view type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogInputs {
    /// Full name of the product
    ProductName,
    /// Quantity/amount of the product to be added
    AmountProduct,
    /// Minimum quantity/amount of the product at the stock/catalog
    MinAmountProduct,
    /// Price to be charged to the user
    ClientPrice,
    /// Price store paid to bought the product
    CostProduct,
}

/// Types of user inputs in [`crate::kinds::Views::Sale`] view type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaleInputs {
    /// Quantity/amount of the product to be sold
    AmountProduct,
    /// Amount of money to make sale
    UserPay,
    /// Client name if it's a loan
    ClientName,
}

/// Types of user inputs in [`crate::kinds::Views::LoanInfo`] view type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoanInputs {
    /// Name debtor to be searched
    DebtorNameLike,
    /// New payment to a loan
    PaymentLoanAmount,
}

/// Types of date picker in [`crate::kinds::Views::LoanInfo`] view type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoanDatePicker {
    /// Date search starts
    StartDatePicker,
    /// Date search ends
    EndDatePicker,
}

/// Modal variant in `loan` view
#[derive(Default)]
pub enum LoanModal {
    #[default]
    /// Payments of a loan
    LoanPayments,
    /// Loan's sale
    LoanSale,
}

/// Types of date picker in [`crate::kinds::Views::SalesInfo`] view type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaleInfoDatePicker {
    /// Date search starts
    StartDatePicker,
    /// Date search ends
    EndDatePicker,
}

/// Events variants that can be send in the app
#[derive(Debug, Clone)]
pub enum AppEvents {
    //App general
    /// External device input Occurred
    ExternalDeviceEventOccurred(iced_native::Event),

    //Catalog view
    /// Event after button main menu was pressed to show catalog module
    ShowCatalog,
    /// An input field element change its state
    CatalogInputChanged(String, CatalogInputs),
    /// In catalog view a product info was requested to fill the catalog
    /// form view data
    CatalogProductInfoRequested(Result<Option<ProductInfo>, String>),
    /// User cancel to add a new product in catalog form view
    CatalogNewRecordCancel,
    /// User agrees to add a new product in catalog form view
    CatalogNewRecordOk,
    /// User agrees to save all the products listed in catalog view
    CatalogSaveAllRecords,
    /// Insertion of all the catalog items was performed
    CatalogNewRecordPerformed(Result<(), String>),
    /// Dropdown of units measurement was used
    CatalogPickListSelected(UnitsMeasurement),
    /// Button to delete an item in the catalog list was pressed
    CatalogRemoveRecordList(String),

    //Sale view
    /// Event after button main menu was pressed to show sale module
    ShowSale,
    /// Info of a product was requested to be sold
    SaleProductInfoRequested(Result<Option<SaleProductInfo>, String>),
    /// An input field element change its state
    SaleInputChanged(String, SaleInputs),
    /// User cancel to add a new product to the sale list
    SaleNewProductCancel,
    /// User agrees to add a new product to the sale list
    SaleNewProductOk,
    /// Cancel the current sale with all the products
    SaleProductsToBuyCancel,
    /// Agrees to bought all the products of the list
    SaleProductsToBuyOk,
    /// Button to delete an item in the sale list was pressed
    SaleRemoveProductToBuyList(String),
    /// Last button to agree the sale
    SaleCreateNewSale,
    /// Insertion of a new sale was performed
    SaleCreateNewSaleRequested(Result<Uuid, String>),
    /// Insertion of a new loan was performed
    SaleCreateNewSaleLoan(Result<(), String>),

    // ToBuy view
    /// Event after button main menu was pressed to show to buy products module
    ToBuyDataRequested,
    /// Search list of products to be bought was triggered
    ToBuyData(Result<Vec<ProductToBuy>, String>),

    // Sale Info view
    /// Event after button main menu was pressed to show sale info module
    ShowSalesInfo,

    // Loan View
    /// Event after loan btn pressed in the main menu
    ShowLoanInfo,
    /// Event to show a date picker
    LoanShowDatePicker(bool, LoanDatePicker),
    /// Event to submit the value selected in the date picker
    LoanSubmitDatePicker(Date, LoanDatePicker),
    /// An input field element change its state
    LoanInputChanged(String, LoanInputs),
    /// Event to start searching loans
    LoanSearchRequested,
    /// Clear data loan view
    LoanClearLoanViewData,
    /// Event is sent after the loans search was performed
    LoanSearchData(Result<Vec<LoanItem>, String>),
    /// Show the main info of the id loan
    LoanShowLoanSale(Uuid),
    /// Show details about payments made to a loan
    LoanShowPaymentsDetails(Uuid),
    /// Event receive data send by the loan repo about
    /// payments of a loan
    LoanPaymentDetailsData(Result<Vec<LoanPayment>, String>),
    /// User request to close the modal shows payment to a loan
    LoanCloseModalPaymentsLoan,
    /// Add new loan event
    LoanAddNewPaymentToLoan,
    /// A new loan paymend was requested
    LoanAddNewPaymentToLoanRequested(Result<(), String>),
    /// Result query sale's products of a loan
    LoanSaleProductsData(Result<Vec<ProductSale>, String>),
    /// Start searching stats
    SaleInfoSearchStats,
    /// Event to show a date picker
    SaleInfoShowDatePicker(bool, SaleInfoDatePicker),
    /// Event to submit the value selected in the date picker
    SaleInfoSubmitDatePicker(Date, SaleInfoDatePicker),
    /// Event after earnings were retrieved
    SaleInfoEarningsData(Result<PgMoney, String>),
    /// Event after sale totals were retrieved
    SaleInfoTotalSales(Result<TotalSales, String>),
    /// Event after loans totals were retrieved
    SaleInfoTotalLoans(Result<TotalLoans, String>),
}
