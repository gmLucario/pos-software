use crate::{
    models::{
        catalog::{LoadProduct, ProductsToBuy},
        sale::SaleProductInfo,
    },
    queries::{
        GET_PRODUCTS_TO_BUY, GET_PRODUCT_CATALOG_INFO, GET_PRODUCT_ID_BY_BARCODE,
        GET_SALE_PRODUCT_INFO, INSERT_PRODUCT_CATALOG,
    },
};

use sqlx::{pool::Pool, postgres::Postgres, types::Uuid};

pub struct ProductRepo {}

impl ProductRepo {
    /// Return products have less than the minimum required
    pub async fn get_products_to_buy(
        connection: &Pool<Postgres>,
    ) -> Result<Vec<ProductsToBuy>, String> {
        let products = sqlx::query_as::<_, ProductsToBuy>(GET_PRODUCTS_TO_BUY)
            .fetch_all(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(products)
    }

    /// Return product info by barcode used in catalog form view
    pub async fn get_product_info_catalog(
        connection: &Pool<Postgres>,
        barcode: String,
    ) -> Result<Option<LoadProduct>, String> {
        let result = sqlx::query_as::<_, LoadProduct>(GET_PRODUCT_CATALOG_INFO)
            .bind(barcode)
            .fetch_optional(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(result)
    }

    /// Save new records to "catalog"
    pub async fn save_products_catalog(
        connection: &Pool<Postgres>,
        products: Vec<LoadProduct>,
    ) -> Result<(), String> {
        for product in products {
            sqlx::query(INSERT_PRODUCT_CATALOG)
                .bind(product.barcode)
                .bind(product.product_name)
                .bind(product.user_price)
                .bind(product.min_amount)
                .bind(product.unit_measurement_id)
                .bind(product.cost)
                .bind(product.current_amount)
                .execute(connection)
                .await
                .map_err(|err| err.to_string())?;
        }
        Ok(())
    }

    /// Get product info by barcode used in sale form view
    pub async fn get_product_info_sale(
        connection: &Pool<Postgres>,
        barcode: String,
    ) -> Result<Option<SaleProductInfo>, String> {
        let result = sqlx::query_as::<_, SaleProductInfo>(GET_SALE_PRODUCT_INFO)
            .bind(barcode)
            .fetch_optional(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(result)
    }

    pub async fn get_product_id(
        connection: &Pool<Postgres>,
        barcode: &str,
    ) -> Result<Uuid, String> {
        let (product_id,): (Uuid,) = sqlx::query_as(GET_PRODUCT_ID_BY_BARCODE)
            .bind(barcode)
            .fetch_one(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(product_id)
    }
}
