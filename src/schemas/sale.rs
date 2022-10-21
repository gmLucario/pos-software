use sqlx::types::BigDecimal;

use crate::{constants::TO_DECIMAL_DIGITS, kinds::UnitsMeasurement, models};

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
