use crate::queries::GET_PRODUCTS_TO_BUY;
use crate::schemas::catalog::ProductsToBuy;

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
}
