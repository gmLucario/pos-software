use std::fmt::Display;

use crate::models::{
    catalog::{LoadProduct, ProductsToBuy},
    sale::SaleProductInfo,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Views {
    Sale,
    SaleAddProductForm,
    SalesInfo,
    ToBuy,
    Catalog,
    CatalogAddRecord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitsMeasurement {
    Kilograms, // 1
    Liters,    // 2
    Pieces,    // 3
}

impl From<i16> for UnitsMeasurement {
    fn from(value: i16) -> Self {
        match value {
            1 => UnitsMeasurement::Kilograms,
            2 => UnitsMeasurement::Liters,
            3 | _ => UnitsMeasurement::Pieces,
        }
    }
}

impl From<UnitsMeasurement> for i16 {
    fn from(unit: UnitsMeasurement) -> Self {
        match unit {
            UnitsMeasurement::Kilograms => 1,
            UnitsMeasurement::Liters => 2,
            UnitsMeasurement::Pieces => 3,
        }
    }
}

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
    pub const ALL: [UnitsMeasurement; 3] = [
        UnitsMeasurement::Kilograms,
        UnitsMeasurement::Liters,
        UnitsMeasurement::Pieces,
    ];
}

impl Default for UnitsMeasurement {
    fn default() -> Self {
        UnitsMeasurement::Pieces
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogInputs {
    ProductName,
    AmountProduct,
    MinAmountProduct,
    ClientPrice,
    CostProduct,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaleInputs {
    AmountProduct,
}

#[derive(Debug, Clone)]
pub enum AppEvents {
    //App general
    InputChangedIgnore(String),
    EventOccurred(iced_native::Event),

    //Catalog view
    CatalogInputChanged(String, CatalogInputs),
    CatalogProductInfoRequested(Result<Option<LoadProduct>, String>),
    CatalogNewRecordCancel,
    CatalogNewRecordOk,
    CatalogSaveAllRecords,
    CatalogNewRecordPerformed(Result<(), String>),
    CatalogPickListSelected(UnitsMeasurement),
    CatalogRemoveRecordList(String),

    //Sale view
    SaleProductInfoRequested(Result<Option<SaleProductInfo>, String>),
    SaleInputChanged(String, SaleInputs),
    SaleNewProductCancel,
    SaleNewProductOk,

    // ToBuy view
    ToBuyData(Result<Vec<ProductsToBuy>, String>),
    ShowSale,
    ShowSalesInfo,
    ShowToBuy,
    ShowCatalog,
    ShowCatalogClosures(Result<(), String>),
}
