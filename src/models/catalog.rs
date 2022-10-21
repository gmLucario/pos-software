use std::str::FromStr;

use sqlx::types::BigDecimal;

use crate::schemas;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct ProductsToBuy {
    pub product_name: String,
    pub amount_to_buy: BigDecimal,
    pub unit_measurement: String,
}

impl ProductsToBuy {
    pub fn get_formatted_item(&self) -> String {
        let amount_to_buy: String = if !self.unit_measurement.eq("Pieza") {
            format!("{:.3}", self.amount_to_buy)
        } else {
            format!("{}", self.amount_to_buy)
        };

        format!(
            "{product_name}: {amount_to_buy} [{unit_measurement}]",
            product_name = self.product_name,
            amount_to_buy = amount_to_buy,
            unit_measurement = self.unit_measurement
        )
    }
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct LoadProduct {
    pub barcode: String,
    pub product_name: String,
    pub user_price: sqlx::postgres::types::PgMoney,
    pub min_amount: BigDecimal,
    pub cost: sqlx::postgres::types::PgMoney,
    pub unit_measurement_id: i16,
    pub current_amount: BigDecimal,
}

impl From<&schemas::catalog::LoadProduct> for LoadProduct {
    fn from(schema: &schemas::catalog::LoadProduct) -> Self {
        let unit_measurement_id: i16 = i16::from(schema.unit_measurement);
        let user_price = sqlx::postgres::types::PgMoney::from_bigdecimal(
            BigDecimal::from_str(&schema.user_price).unwrap(),
            2,
        )
        .unwrap();
        let cost = sqlx::postgres::types::PgMoney::from_bigdecimal(
            BigDecimal::from_str(&schema.cost).unwrap(),
            2,
        )
        .unwrap();
        let min_amount = BigDecimal::from_str(&schema.min_amount).unwrap();
        let current_amount = BigDecimal::from_str(&schema.amount).unwrap();

        Self {
            barcode: schema.barcode.to_string(),
            product_name: schema.product_name.to_string(),
            user_price,
            min_amount,
            cost,
            unit_measurement_id,
            current_amount,
        }
    }
}
