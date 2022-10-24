use std::str::FromStr;

use iced::button;
use sqlx::{postgres::types::PgMoney, types::BigDecimal};

use crate::{
    constants::{PGMONEY_DECIMALS, TO_DECIMAL_DIGITS},
    kinds::UnitsMeasurement,
    models,
};

#[derive(Default, Debug, Clone)]
pub struct ProductToAdd {
    pub barcode: String,
    pub product_name: String,
    pub price: String,
    pub unit_measurement: UnitsMeasurement,
    pub amount: String,
    pub total_amount: BigDecimal,
}

impl ProductToAdd {
    pub fn reset_values(&mut self) {
        *self = Self::default();
    }
}

impl From<models::sale::SaleProductInfo> for ProductToAdd {
    fn from(model: models::sale::SaleProductInfo) -> Self {
        let unit_measurement = UnitsMeasurement::from(model.unit_measurement_id);

        Self {
            barcode: model.barcode,
            product_name: model.product_name,
            price: model.price.to_bigdecimal(TO_DECIMAL_DIGITS).to_string(),
            unit_measurement,
            amount: model.amount.to_string(),
            total_amount: model.total_amount,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProductList {
    pub product_name: String,
    pub amount: BigDecimal,
    pub price: PgMoney,
    pub delete_btn_state: button::State,
}

impl From<&ProductToAdd> for ProductList {
    fn from(schema: &ProductToAdd) -> Self {
        let amount = BigDecimal::from_str(&schema.amount).unwrap();
        let price = BigDecimal::from_str(&schema.price).unwrap();
        let price = PgMoney::from_bigdecimal(price * &amount, PGMONEY_DECIMALS).unwrap();

        Self {
            product_name: schema.product_name.to_string(),
            delete_btn_state: button::State::new(),
            amount,
            price,
        }
    }
}
