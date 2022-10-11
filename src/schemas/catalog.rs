#[derive(sqlx::FromRow, Debug, Clone)]
pub struct ProductsToBuy {
    pub product_name: String,
    pub amount_to_buy: i64,
}
