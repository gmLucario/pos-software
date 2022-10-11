use crate::schemas::catalog::ProductsToBy;

#[derive(Debug, Clone)]
pub enum Views {
    Sale,
    SalesInfo,
    ToBuy,
    Catalog,
}

#[derive(Debug, Clone)]
pub enum AppEvents {
    ToBuyData(Result<Vec<ProductsToBy>, String>),
    ShowSale,
    ShowSalesInfo,
    ShowToBuy,
    ShowCatalog,
}
