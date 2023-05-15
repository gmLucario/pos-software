//! Interaction with the database related with a sale

use std::cmp::Ordering;

use sqlx::{
    postgres::types::PgMoney,
    types::{BigDecimal, Uuid},
    Pool, Postgres,
};

use crate::{
    data::product_repo::ProductRepo,
    models::sale::{CatalogAmount, ProductSale, Sale, TotalSales},
    queries::{
        CREATE_OPERATION_FROM_CATALOG, DELETE_CATALOG_RECORD, GET_EARNINGS,
        GET_PRODUCTS_CATALOG_UPDATE_SALE, GET_PRODUCTS_SALE, GET_SALE_TOTAL, INSERT_NEW_SALE,
        INSERT_NEW_SALE_OPERATION, UPDATE_CATALOG_AMOUNT,
    },
};

/// Struct to group the functionality related with
/// the interaction of the database and a sale
pub struct SaleRepo {}

impl SaleRepo {
    /// Handle all the logic to run a new sale
    pub async fn process_new_sale_flow(
        connection: &Pool<Postgres>,
        sale: Sale,
    ) -> Result<Uuid, String> {
        let sale_id: Uuid = Self::save_new_sale(connection, &sale.client_payment).await?;

        for product in sale.products {
            let product_id = ProductRepo::get_product_id(connection, &product.barcode).await?;

            let mut products: Vec<CatalogAmount> =
                sqlx::query_as::<_, CatalogAmount>(GET_PRODUCTS_CATALOG_UPDATE_SALE)
                    .bind(product_id)
                    .bind(&product.amount)
                    .fetch_all(connection)
                    .await
                    .map_err(|err| err.to_string())?;

            Self::process_catalog_amounts(&mut products, &product.amount)?;
            Self::update_related_sale_records(connection, &products, &sale_id).await?
        }

        Ok(sale_id)
    }

    /// Insert a new sale
    async fn save_new_sale(
        connection: &Pool<Postgres>,
        client_payment: &PgMoney,
    ) -> Result<Uuid, String> {
        let (sale_id,): (Uuid,) = sqlx::query_as(INSERT_NEW_SALE)
            .bind(client_payment)
            .fetch_one(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(sale_id)
    }

    /// Update the product amount in the catalog based on the amount sold
    fn process_catalog_amounts(
        products: &mut [CatalogAmount],
        amount: &BigDecimal,
    ) -> Result<(), String> {
        let mut current_amount: BigDecimal = amount.clone();
        let zero_amount = BigDecimal::default();
        for product in products.iter_mut() {
            if current_amount <= BigDecimal::default() {
                return Ok(());
            }

            (current_amount, product.amount) = match product.amount.cmp(&current_amount) {
                Ordering::Less | Ordering::Equal => {
                    (current_amount - product.amount.clone(), zero_amount.clone())
                }
                Ordering::Greater => (zero_amount.clone(), product.amount.clone() - current_amount),
            };
        }

        Ok(())
    }

    /// Update stock products based on each item of the sale
    /// Each item will be an operation of type sale
    async fn update_related_sale_records(
        connection: &Pool<Postgres>,
        products: &Vec<CatalogAmount>,
        sale_id: &Uuid,
    ) -> Result<(), String> {
        let zero_amount = BigDecimal::default();

        for product in products {
            let operation_id: Uuid = Self::create_new_operation(connection, product).await?;

            Self::create_new_sale_operation(connection, sale_id, &operation_id).await?;

            if product.amount == zero_amount {
                Self::delete_catalog_record(connection, &product.catalog_id).await?;
            } else {
                Self::update_catalog_amount(connection, &product.catalog_id, &product.amount)
                    .await?;
            }
        }

        Ok(())
    }

    /// Update current_amount field of a catalog item
    async fn update_catalog_amount(
        connection: &Pool<Postgres>,
        catalog_id: &Uuid,
        amount: &BigDecimal,
    ) -> Result<(), String> {
        sqlx::query(UPDATE_CATALOG_AMOUNT)
            .bind(catalog_id)
            .bind(amount)
            .execute(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(())
    }

    /// Delete the product from the catalog
    async fn delete_catalog_record(
        connection: &Pool<Postgres>,
        catalog_id: &Uuid,
    ) -> Result<(), String> {
        sqlx::query(DELETE_CATALOG_RECORD)
            .bind(catalog_id)
            .execute(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(())
    }

    /// Create a new sale operation record
    async fn create_new_operation(
        connection: &Pool<Postgres>,
        product: &CatalogAmount,
    ) -> Result<Uuid, String> {
        let (operation_id,): (Uuid,) = sqlx::query_as(CREATE_OPERATION_FROM_CATALOG)
            .bind(product.catalog_id)
            .bind(&product.amount)
            .fetch_one(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(operation_id)
    }

    /// Link the sale with the operation
    async fn create_new_sale_operation(
        connection: &Pool<Postgres>,
        sale_id: &Uuid,
        operation_id: &Uuid,
    ) -> Result<(), String> {
        sqlx::query(INSERT_NEW_SALE_OPERATION)
            .bind(sale_id)
            .bind(operation_id)
            .execute(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(())
    }

    /// Return list produts of a sale
    pub async fn get_products_sale(
        connection: &Pool<Postgres>,
        sale_id: Uuid,
    ) -> Result<Vec<ProductSale>, String> {
        let products = sqlx::query_as::<_, ProductSale>(GET_PRODUCTS_SALE)
            .bind(sale_id)
            .fetch_all(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(products)
    }

    /// Get total earnings of a period
    pub async fn get_total_earnings(
        connection: &Pool<Postgres>,
        start_date: String,
        end_date: String,
    ) -> Result<PgMoney, String> {
        let (earnings,): (PgMoney,) = sqlx::query_as(GET_EARNINGS)
            .bind(start_date)
            .bind(end_date)
            .fetch_one(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(earnings)
    }

    /// Get total stats sales
    pub async fn get_total_sales(
        connection: &Pool<Postgres>,
        start_date: String,
        end_date: String,
    ) -> Result<TotalSales, String> {
        let totals: TotalSales = sqlx::query_as(GET_SALE_TOTAL)
            .bind(start_date)
            .bind(end_date)
            .fetch_one(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(totals)
    }
}
