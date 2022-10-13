use crate::models::catalog::{LoadProduct, ProductsToBuy};
use crate::queries::{GET_PRODUCTS_TO_BUY, GET_PRODUCT_CATALOG_INFO};

use sqlx::{pool::Pool, postgres::Postgres};

pub struct ProductRepo {}

impl ProductRepo {
    /// Return products have less than the minimum required
    pub async fn get_products_to_buy(
        connection: &Pool<Postgres>,
    ) -> Result<Vec<ProductsToBuy>, String> {
        let products = sqlx::query_as::<_, ProductsToBuy>(GET_PRODUCTS_TO_BUY)
            .fetch_all(connection)
            .await
            .map_err(|_| String::new())?;

        Ok(products)
    }

    /// Return product info by barcode
    pub async fn get_product_info_catalog(
        connection: &Pool<Postgres>,
        barcode: String,
    ) -> Result<Option<LoadProduct>, String> {
        let result = sqlx::query_as::<_, LoadProduct>(GET_PRODUCT_CATALOG_INFO)
            .bind(barcode)
            .fetch_optional(connection)
            .await
            .map_err(|_| String::new())?;

        Ok(result)
    }
}
