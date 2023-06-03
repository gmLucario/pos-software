//! Interaction with the database related with a product

use sqlx::{
    postgres::PgPool,
    types::{BigDecimal, Uuid},
};

use crate::{
    db::queries::{
        GET_CATALOG_PRODUCTS, GET_PRODUCTS_CATALOG_UPDATE_SALE, GET_PRODUCTS_TO_BUY,
        GET_PRODUCT_CATALOG_INFO, GET_PRODUCT_ID_BY_BARCODE, GET_SALE_PRODUCT_INFO,
        INSERT_PRODUCT_CATALOG,
    },
    errors::AppError,
    models::{
        catalog::{LoadProduct, ProductAmount, ProductInfo, ProductToBuy},
        sale::{CatalogAmount, SaleProductInfo},
    },
    result::AppResult,
};

/// Gets a [crate::errors::AppError] of type [crate::errors::ErrorType::DbError] with a custom `raw_msg`
fn get_db_error(raw_msg: &str, msg: &str, function_name: &str) -> AppError {
    AppError::db_error(
        &format!("src/repo/product_repo.rs::{function_name}"),
        msg,
        raw_msg,
    )
}

/// Gets product id by barcode
pub async fn get_product_id(connection: &PgPool, barcode: &str) -> AppResult<Uuid> {
    sqlx::query_as::<_, (Uuid,)>(GET_PRODUCT_ID_BY_BARCODE)
        .bind(barcode)
        .fetch_one(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "error al obtener el product_id",
                "get_product_id",
            )
        })
        .map(|(uuid,)| uuid)
}

/// Retrieves the current amount of each product in the catalog like `product_name_like`
pub async fn get_products_catalog_like(
    connection: &PgPool,
    product_name_like: String,
    page: i64,
) -> AppResult<Vec<ProductAmount>> {
    sqlx::query_as::<_, ProductAmount>(GET_CATALOG_PRODUCTS)
        .bind(product_name_like)
        .bind(page)
        .fetch_all(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "error retrieving the current amount of each product in the catalog",
                "get_products_catalog_like",
            )
        })
}

/// Retrieves the products to be bought that matches `product_name_like`
pub async fn get_products_tobuy_like(
    connection: &PgPool,
    product_name_like: String,
) -> AppResult<Vec<ProductToBuy>> {
    sqlx::query_as::<_, ProductToBuy>(GET_PRODUCTS_TO_BUY)
        .bind(product_name_like)
        .fetch_all(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "error retrieving the products to be bought",
                "get_products_tobuy_like",
            )
        })
}

/// Returns product info by barcode used in catalog form view
pub async fn get_product_info_catalog(
    connection: &PgPool,
    barcode: String,
) -> AppResult<Option<ProductInfo>> {
    sqlx::query_as::<_, ProductInfo>(GET_PRODUCT_CATALOG_INFO)
        .bind(barcode)
        .fetch_optional(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "Error retrieving product info for catalog form",
                "get_product_info_catalog",
            )
        })
}

/// Saves new records to "catalog"
pub async fn save_products_catalog(
    connection: &PgPool,
    products: Vec<LoadProduct>,
) -> AppResult<()> {
    let mut barcodes_not_inserted: Vec<String> = vec![];

    for product in products {
        let result = sqlx::query(INSERT_PRODUCT_CATALOG)
            .bind(&product.barcode)
            .bind(product.product_name)
            .bind(product.user_price)
            .bind(product.min_amount)
            .bind(product.unit_measurement_id)
            .bind(product.cost)
            .bind(product.current_amount)
            .execute(connection)
            .await;

        if result.is_err() {
            barcodes_not_inserted.push(product.barcode)
        }
    }

    if !barcodes_not_inserted.is_empty() {
        return Err(get_db_error(
            "Some barcodes were not inserted",
            "Some barcodes were not inserted",
            "save_products_catalog",
        ));
    }

    Ok(())
}

/// Gets product info by barcode used in sale form view
pub async fn get_product_info_sale(
    connection: &PgPool,
    barcode: String,
) -> AppResult<Option<SaleProductInfo>> {
    sqlx::query_as::<_, SaleProductInfo>(GET_SALE_PRODUCT_INFO)
        .bind(barcode)
        .fetch_optional(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "Error retrieving product info by barcode used in sale form view",
                "get_product_info_sale",
            )
        })
}

/// Gets the current state info related to product_id
pub async fn get_current_state_products(
    connection: &PgPool,
    product_id: &Uuid,
    amount: &BigDecimal,
) -> AppResult<Vec<CatalogAmount>> {
    sqlx::query_as::<_, CatalogAmount>(GET_PRODUCTS_CATALOG_UPDATE_SALE)
        .bind(product_id)
        .bind(amount)
        .fetch_all(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "Error al obtener la info de ese producto en catalogo",
                "get_current_state_products",
            )
        })
}
