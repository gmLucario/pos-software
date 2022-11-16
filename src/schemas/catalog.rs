//! Data structures to get user inputs related to catalog view

use crate::{constants::TO_DECIMAL_DIGITS, kinds::UnitsMeasurement, models};

/// Represents user input to load a new product to the catalog
#[derive(Default, Debug, Clone)]
pub struct LoadProduct {
    /// `barcode` of the product
    pub barcode: String,
    /// full name of the product
    pub product_name: String,
    /// Price to be charged to the user
    pub user_price: String,
    /// Amount/Quantity of the product
    pub amount: String,
    /// Unit of measurement of the product: kg, lts or pieces
    pub unit_measurement: UnitsMeasurement,
    /// Min amount the catalog must have
    pub min_amount: String,
    /// Product price
    pub cost: String,
}

impl LoadProduct {
    /// Get an "unique" id of the input product based on its fields
    pub fn get_id(&self) -> String {
        format!("{}@{}@{}", self.barcode, self.amount, self.cost)
    }
}

impl From<models::catalog::ProductInfo> for LoadProduct {
    /// From model to schema
    fn from(model: models::catalog::ProductInfo) -> Self {
        let unit_measurement = UnitsMeasurement::from(model.unit_measurement_id);

        Self {
            amount: "1".to_string(),
            unit_measurement,
            barcode: model.barcode,
            product_name: model.product_name,
            user_price: model
                .user_price
                .to_bigdecimal(TO_DECIMAL_DIGITS)
                .to_string(),
            min_amount: model.min_amount.to_string(),
            cost: model.cost.to_bigdecimal(TO_DECIMAL_DIGITS).to_string(),
        }
    }
}
