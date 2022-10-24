use sqlx::types::BigDecimal;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct SaleProductInfo {
    pub barcode: String,
    pub product_name: String,
    pub price: sqlx::postgres::types::PgMoney,
    pub amount: BigDecimal,
    pub total_amount: BigDecimal,
    pub unit_measurement_id: i16,
}
