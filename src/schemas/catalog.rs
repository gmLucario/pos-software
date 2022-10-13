use crate::{kinds::UnitsMeasurement, models};

#[derive(Default, Debug, Clone)]
pub struct LoadProduct {
    pub barcode: String,
    pub product_name: String,
    pub user_price: String,
    pub amount: String,
    pub unit_measurement: UnitsMeasurement,
    pub min_amount: String,
    pub cost: String,
}

impl From<models::catalog::LoadProduct> for LoadProduct {
    fn from(model: models::catalog::LoadProduct) -> Self {
        let unit_measurement = if model.unit_measurement_id.eq(&1) {
            UnitsMeasurement::Kilograms
        } else if model.unit_measurement_id.eq(&2) {
            UnitsMeasurement::Liters
        } else {
            UnitsMeasurement::Pieces
        };

        LoadProduct {
            amount: String::new(),
            unit_measurement,
            barcode: model.barcode,
            product_name: model.product_name,
            user_price: model.user_price.to_bigdecimal(2).to_string(),
            min_amount: model.min_amount.to_string(),
            cost: model.cost.to_bigdecimal(2).to_string(),
        }
    }
}
