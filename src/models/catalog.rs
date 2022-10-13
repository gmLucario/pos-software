use sqlx::types::BigDecimal;

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
}
