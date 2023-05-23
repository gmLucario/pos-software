//! Events variants that can be sent in the app

use std::collections::BTreeMap;

use custom_crates::widgets::toast;
use iced::{widget::scrollable::RelativeOffset, Event};
use iced_aw::date_picker::Date;
use sqlx::{postgres::types::PgMoney, types::Uuid};

use crate::{
    kinds::{AppDatePicker, AppModule, ModalView, OnScroll, PickList, TextInput, View},
    models::{
        catalog::{ProductAmount, ProductInfo, ProductToBuy},
        loan::{LoanInfo, LoanPayment, TotalLoans},
        sale::{ProductSale, SaleProductInfo, TotalSales},
    },
    result::AppResult,
};

#[derive(Debug, Clone)]
/// App events
pub enum AppEvent {
    //App general
    /// External device input Occurred
    ExternalDeviceEventOccurred(Event),
    /// Event to change the current app module
    ChangeAppModule(AppModule, View),
    /// Event to change the current app view
    ChangeView(View),
    /// Event to change the current modal view
    ChangeModalView(ModalView),
    /// Event to show the modal
    ShowModal,
    /// Event to hide the modal
    HideModal,
    /// Event to add a new toast message to the queue
    AddToast(toast::Status, String),
    /// Event to remove a toast message to the queue
    CloseToast(usize),
    /// Set default data for current view
    SetDefaultDataView,
    ///A [`iced::widget::TextInput`] field changed
    TextInputChanged(String, TextInput),
    /// A picklist value was selected
    PickListSelected(PickList),
    /// Event to show a date picker
    ShowDatePicker(bool, AppDatePicker),
    /// Event to submit the value selected in the date picker
    SubmitDatePicker(Date, AppDatePicker),
    /// scroll widget was used
    ScrollScrolled(OnScroll, RelativeOffset),

    // Catalog module view
    // Result obtained in the query to retrieve products in the catalog
    CatalogProductsData(AppResult<Vec<ProductAmount>>),
    /// Search a product info to fill catalog form
    CatalogRequestProductInfoForm,
    /// Product info was requested to fill the catalog form
    CatalogProductInfoFormRequested(AppResult<Option<ProductInfo>>),
    /// User cancel to add a new product in catalog form view
    CatalogNewRecordListTobeSavedCancel,
    /// User agrees to add a new product in catalog form view
    CatalogNewRecordListTobeSavedOk(bool),
    /// User wants to edit an item from the products to be added in the catalog
    CatalogEditRecordListTobeSaved(String),
    /// User wants to remove an item from the products to be added in the catalog
    CatalogRemoveRecordListTobeSaved(String),
    /// User agrees to save all the products listed in catalog view
    CatalogSaveAllRecords,
    /// Insertion of all the catalog items was performed
    CatalogSaveAllRecordsPerformed(AppResult<()>),

    // Sale module view
    /// Info of a product was requested to be sold
    SaleProductInfoRequested(AppResult<Option<SaleProductInfo>>),
    /// Button to delete an item in the sale list was pressed
    SaleResetProductsToBeSold,
    /// Button to edit an item in the sale list was pressed
    SaleEditProductToBeSold(SaleProductInfo),
    /// Button to delete an item in the sale list was pressed
    SaleRemoveProductToBeSold(String),
    /// Sale form edit/add product ok
    SaleProductAddEditFormOk(SaleProductInfo, bool),
    /// Last button to agree the sale
    SaleCreateNewSale,
    /// Insertion of a new sale was performed
    SaleCreateNewSaleRequested(AppResult<Uuid>),
    /// Insertion of a new loan was performed
    SaleCreateNewSaleLoan(AppResult<()>),

    // Loan module view
    /// Show the main info of the id loan
    LoanShowLoanSale(Uuid),
    /// Show details about payments made to a loan
    LoanShowPaymentsDetails(Uuid),
    /// Event to start searching loans
    LoanSearchRequested,
    /// Event is sent after the loans search was performed
    LoanSearchData(AppResult<BTreeMap<String, LoanInfo>>),
    /// Result query sale's products of a loan
    LoanSaleProductsData(AppResult<Vec<ProductSale>>),
    /// Event receive data send by the loan repo about
    /// payments of a loan
    LoanPaymentDetailsData(AppResult<Vec<LoanPayment>>),
    /// Add new payment loan event
    LoanAddNewPaymentToLoan,
    /// A new loan paymend was requested
    LoanAddNewPaymentToLoanRequested(AppResult<()>),

    // To Buy
    /// Search list of products to be bought was triggered
    ToBuySearchData(AppResult<Vec<ProductToBuy>>),

    // Stats module
    /// Start searching stats
    SaleInfoSearchStats,
    /// Event after earnings were retrieved
    SaleInfoEarningsData(AppResult<PgMoney>),
    /// Event after sale totals were retrieved
    SaleInfoTotalSales(AppResult<TotalSales>),
    /// Event after loans totals were retrieved
    SaleInfoTotalLoans(AppResult<TotalLoans>),
}
