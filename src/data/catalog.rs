use crate::queries::GET_PRODUCTS_TO_BY;
use crate::schemas::catalog::ProductsToBy;

use sqlx::{pool::Pool, postgres::Postgres};

pub struct CatalogRepo {}

impl CatalogRepo {
    /// Return products have less than the minimum required
    pub async fn get_products_to_buy(
        connection: &Pool<Postgres>,
    ) -> Result<Vec<ProductsToBy>, String> {
        let products = sqlx::query_as::<_, ProductsToBy>(GET_PRODUCTS_TO_BY)
            .fetch_all(connection)
            .await
            .map_err(|_| String::new())?;

        Ok(products)
    }
}
