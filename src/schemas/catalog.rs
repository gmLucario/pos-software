#[derive(sqlx::FromRow, Debug, Clone)]
pub struct ProductsToBy {
    pub product_name: String,
    pub amount_to_buy: i64,
}
