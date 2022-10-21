use std::str::FromStr;

use sqlx::{postgres::types::PgMoney, types::BigDecimal};

use crate::schemas;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct SaleProductInfo {
    pub barcode: String,
    pub product_name: String,
    pub price: sqlx::postgres::types::PgMoney,
    pub amount: BigDecimal,
    pub total_amount: BigDecimal,
    pub unit_measurement_id: i16,
}

impl From<&schemas::sale::ProductToAdd> for SaleProductInfo {
    fn from(schema: &schemas::sale::ProductToAdd) -> Self {
        let amount = BigDecimal::from_str(&schema.amount).unwrap();

        let price = BigDecimal::from_str(&schema.price).unwrap() * &amount;
        let price = PgMoney::from_bigdecimal(price, 2).unwrap();

        let unit_measurement_id = i16::from(schema.unit_measurement);

        Self {
            barcode: schema.barcode.to_string(),
            product_name: schema.product_name.to_string(),
            price,
            amount,
            total_amount: schema.total_amount.clone(),
            unit_measurement_id,
        }
    }
}
