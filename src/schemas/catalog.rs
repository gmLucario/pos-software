use iced::button;

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

    pub edit_button_state: button::State,
}
impl LoadProduct {
    pub fn get_id(&self) -> String {
        format!("{}@{}@{}", self.barcode, self.amount, self.cost)
    }

    pub fn format_to_user(&self) -> String {
        format!(
            "{barcode}:{product}  |   {amount}[{units}]   |   ${cost}{emoji_cost}   |   ${user_price}",
            barcode = self.barcode,
            product = self.product_name,
            amount = self.amount,
            units = self.unit_measurement,
            cost = self.cost,
            emoji_cost = '\u{1F4B5}',
            user_price = self.user_price,
        )
    }
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

            edit_button_state: button::State::new(),
        }
    }
}
