use crate::schemas::catalog::ProductsToBuy;

#[derive(Debug, Clone)]
pub enum Views {
    Sale,
    SalesInfo,
    ToBuy,
    Catalog,
}

#[derive(Debug, Clone)]
pub enum AppEvents {
    ToBuyData(Result<Vec<ProductsToBuy>, String>),
    ShowSale,
    ShowSalesInfo,
    ShowToBuy,
    ShowCatalog,
}
