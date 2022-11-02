use std::str::FromStr;

use sqlx::{
    postgres::types::PgMoney,
    types::{BigDecimal, Uuid},
};

use crate::{constants::PGMONEY_DECIMALS, schemas::sale::SaleInfo};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct SaleProductInfo {
    pub barcode: String,
    pub product_name: String,
    pub price: PgMoney,
    pub amount: BigDecimal,
    pub total_amount: BigDecimal,
    pub unit_measurement_id: i16,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct CatalogAmount {
    pub catalog_id: Uuid,
    pub amount: BigDecimal,
    pub cost: PgMoney,
}

#[derive(Debug, Clone)]
pub struct SaleProductAmount {
    pub barcode: String,
    pub amount: BigDecimal,
}

#[derive(Debug, Clone)]
pub struct Sale {
    pub client_payment: PgMoney,
    pub products: Vec<SaleProductAmount>,
}

impl From<&SaleInfo> for Sale {
    fn from(schema: &SaleInfo) -> Self {
        let client_payment = BigDecimal::from_str(&schema.client_pay).unwrap();
        let client_payment = PgMoney::from_bigdecimal(client_payment, PGMONEY_DECIMALS).unwrap();
        let barcodes: Vec<SaleProductAmount> = schema
            .products
            .iter()
            .map(|(k, v)| SaleProductAmount {
                barcode: k.to_string(),
                amount: v.amount.clone(),
            })
            .collect();

        Self {
            client_payment,
            products: barcodes,
        }
    }
}
