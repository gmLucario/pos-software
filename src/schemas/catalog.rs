use iced::button;

use crate::{constants::TO_DECIMAL_DIGITS, kinds::UnitsMeasurement, models};

#[derive(Default, Debug, Clone)]
pub struct LoadProduct {
    pub barcode: String,
    pub product_name: String,
    pub user_price: String,
    pub amount: String,
    pub unit_measurement: UnitsMeasurement,
    pub min_amount: String,
    pub cost: String,

    pub edit_button_state: button::State,
}
impl LoadProduct {
    pub fn get_id(&self) -> String {
        format!("{}@{}@{}", self.barcode, self.amount, self.cost)
    }
}

impl From<models::catalog::LoadProduct> for LoadProduct {
    fn from(model: models::catalog::LoadProduct) -> Self {
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

            edit_button_state: button::State::new(),
        }
    }
}
