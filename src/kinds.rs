use std::fmt::Display;

use crate::models::catalog::{LoadProduct, ProductsToBuy};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Views {
    Sale,
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

#[derive(Debug, Clone)]
pub enum AppEvents {
    //App general
    InputChangedIgnore(String),

    //Catalog view
    EventOccurred(iced_native::Event),
    InputChangedCatalog(String, CatalogInputs),
    CatalogAddRecordData(Result<Option<LoadProduct>, String>),
    CatalogNewRecordCancel,
    CatalogNewRecordOk,
    CatalogPickListSelected(UnitsMeasurement),
    RemoveRecordList(String),
    SaveAllRecords,

    // ToBuy view
    ToBuyData(Result<Vec<ProductsToBuy>, String>),
    ShowSale,
    ShowSalesInfo,
    ShowToBuy,
    ShowCatalog,
}
