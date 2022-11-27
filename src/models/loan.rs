//! Structs represent queries results about loans

use sqlx::{
    postgres::types::PgMoney,
    types::{chrono::NaiveDateTime, Uuid},
};

#[derive(sqlx::FromRow, Debug, Clone)]
/// Represents each loan record and its main info
pub struct LoanItem {
    pub id: Uuid,
    pub loan_balance: PgMoney,
    pub name_debtor: String,
    pub sold_at: NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug, Clone)]
/// Represents each payment made to a loan
pub struct LoanPayment {
    pub money_amount: PgMoney,
    pub payed_at: NaiveDateTime,
}
