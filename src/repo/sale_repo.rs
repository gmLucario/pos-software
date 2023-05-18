//! Interaction with the database related with a sale

use std::cmp::Ordering;

use sqlx::{
    postgres::{types::PgMoney, PgPool},
    types::{BigDecimal, Uuid},
};

use crate::{
    db::queries::{
        CREATE_OPERATION_FROM_CATALOG, DELETE_CATALOG_RECORD, GET_EARNINGS, GET_PRODUCTS_SALE,
        GET_SALE_TOTAL, INSERT_NEW_SALE, INSERT_NEW_SALE_OPERATION, UPDATE_CATALOG_AMOUNT,
    },
    errors::AppError,
    models::sale::{CatalogAmount, ProductSale, Sale, TotalSales},
    repo::product_repo,
    result::AppResult,
};

/// Get a [crate::errors::AppError] of type [crate::errors::ErrorType::DbError] with a custom `raw_msg`
fn get_db_error(raw_msg: &str, msg: &str, function_name: &str) -> AppError {
    AppError::db_error(
        &format!("src/repo/sale_repo.rs::{function_name}"),
        msg,
        raw_msg,
    )
}

/// Handle all the logic to run a new sale
pub async fn process_new_sale_flow(connection: &PgPool, sale: Sale) -> AppResult<Uuid> {
    let sale_id: Uuid = save_new_sale(connection, &sale.client_payment).await?;

    for product in sale.products {
        let product_id = product_repo::get_product_id(connection, &product.barcode).await?;

        let mut products =
            product_repo::get_current_state_products(connection, &product_id, &product.amount)
                .await?;

        process_catalog_amounts(&mut products, &product.amount);
        update_related_sale_records(connection, &products, &sale_id).await?
    }

    Ok(sale_id)
}

/// Insert a new sale
async fn save_new_sale(connection: &PgPool, client_payment: &PgMoney) -> AppResult<Uuid> {
    sqlx::query_as::<_, (Uuid,)>(INSERT_NEW_SALE)
        .bind(client_payment)
        .fetch_one(connection)
        .await
        .map_err(|err| get_db_error(&err.to_string(), "Error al crear la venta", "save_new_sale"))
        .map(|(sale_id,)| sale_id)
}

/// Return list produts of a sale
pub async fn get_products_sale(connection: &PgPool, sale_id: Uuid) -> AppResult<Vec<ProductSale>> {
    sqlx::query_as::<_, ProductSale>(GET_PRODUCTS_SALE)
        .bind(sale_id)
        .fetch_all(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "No se pudo obtener los productos de la venta",
                "get_products_sale",
            )
        })
}

/// Update stock products based on each item of the sale
/// Update the product amount in the catalog based on the amount sold
fn process_catalog_amounts(products: &mut [CatalogAmount], amount: &BigDecimal) {
    let mut current_amount: BigDecimal = amount.clone();
    let zero_amount = BigDecimal::default();
    for product in products.iter_mut() {
        if current_amount <= BigDecimal::default() {
            return;
        }

        (current_amount, product.amount) = match product.amount.cmp(&current_amount) {
            Ordering::Less | Ordering::Equal => {
                (current_amount - product.amount.clone(), zero_amount.clone())
            }
            Ordering::Greater => (zero_amount.clone(), product.amount.clone() - current_amount),
        };
    }
}

/// Each item will be an operation of type sale
async fn update_related_sale_records(
    connection: &PgPool,
    products: &Vec<CatalogAmount>,
    sale_id: &Uuid,
) -> AppResult<()> {
    let zero_amount = BigDecimal::default();

    for product in products {
        let operation_id: Uuid = create_new_operation(connection, product).await?;

        create_new_sale_operation(connection, sale_id, &operation_id).await?;

        if product.amount == zero_amount {
            delete_catalog_record(connection, &product.catalog_id).await?;
        } else {
            update_catalog_amount(connection, &product.catalog_id, &product.amount).await?;
        }
    }

    Ok(())
}
/// Create a new sale operation record
async fn create_new_operation(connection: &PgPool, product: &CatalogAmount) -> AppResult<Uuid> {
    sqlx::query_as::<_, (Uuid,)>(CREATE_OPERATION_FROM_CATALOG)
        .bind(product.catalog_id)
        .bind(&product.amount)
        .fetch_one(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "Error al crear una operacion de la venta",
                "create_new_sale_operation",
            )
        })
        .map(|(operation_id,)| operation_id)
}

/// Link the sale with the operation
async fn create_new_sale_operation(
    connection: &PgPool,
    sale_id: &Uuid,
    operation_id: &Uuid,
) -> AppResult<()> {
    sqlx::query(INSERT_NEW_SALE_OPERATION)
        .bind(sale_id)
        .bind(operation_id)
        .execute(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "Error al crear una operacion de la venta",
                "create_new_sale_operation",
            )
        })
        .map(|_| ())
}

/// Delete the product from the catalog
async fn delete_catalog_record(connection: &PgPool, catalog_id: &Uuid) -> AppResult<()> {
    sqlx::query(DELETE_CATALOG_RECORD)
        .bind(catalog_id)
        .execute(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "Error al eliminar el producto del catalogo",
                "delete_catalog_record",
            )
        })
        .map(|_| ())
}

/// Update current_amount field of a catalog item
async fn update_catalog_amount(
    connection: &PgPool,
    catalog_id: &Uuid,
    amount: &BigDecimal,
) -> AppResult<()> {
    sqlx::query(UPDATE_CATALOG_AMOUNT)
        .bind(catalog_id)
        .bind(amount)
        .execute(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "Error al aztualizar la cantidad en el catalogo",
                "update_catalog_amount",
            )
        })
        .map(|_| ())
}

/// Get total earnings of a period
pub async fn get_total_earnings(
    connection: &PgPool,
    start_date: String,
    end_date: String,
) -> AppResult<PgMoney> {
    sqlx::query_as::<_, (PgMoney,)>(GET_EARNINGS)
        .bind(start_date)
        .bind(end_date)
        .fetch_one(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "Error al obtner el total de las ganancias",
                "get_total_earnings",
            )
        })
        .map(|(earnings,)| earnings)
}

/// Get total stats sales
pub async fn get_total_sales(
    connection: &PgPool,
    start_date: String,
    end_date: String,
) -> AppResult<TotalSales> {
    sqlx::query_as(GET_SALE_TOTAL)
        .bind(start_date)
        .bind(end_date)
        .fetch_one(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "Error al obtner el total de las ganancias",
                "get_total_sales",
            )
        })
}
