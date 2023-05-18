use sqlx::{
    postgres::{types::PgMoney, PgPool},
    types::Uuid,
};

use crate::{
    db::queries::{
        GET_LOAN_LIST, GET_LOAN_TOTAL, GET_PAYMENTS_LOAN, INSERT_NEW_LOAN, INSERT_NEW_PAYMENT_LOAN,
    },
    errors::AppError,
    models::{
        loan::{LoanItem, LoanPayment, TotalLoans},
        sale::SaleLoan,
    },
    result::AppResult,
};

/// Get a [crate::errors::AppError] of type [crate::errors::ErrorType::DbError] with a custom `raw_msg`
fn get_db_error(raw_msg: &str, msg: &str, function_name: &str) -> AppError {
    AppError::db_error(
        &format!("src/repo/loan_repo.rs::{function_name}"),
        msg,
        raw_msg,
    )
}

/// Save a new sale loan
pub async fn save_new_loan(connection: &PgPool, loan: SaleLoan) -> AppResult<()> {
    sqlx::query(INSERT_NEW_LOAN)
        .bind(loan.sale_id)
        .bind(loan.money_amount)
        .bind(&loan.name_debtor)
        .execute(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "error al crear un prestamo",
                "save_new_loan",
            )
        })
        .map(|_| ())
}

/// Return the loans that match the data filter
pub async fn get_loans_by_debtor_name(
    connection: &PgPool,
    name_debtor: String,
) -> AppResult<Vec<LoanItem>> {
    sqlx::query_as::<_, LoanItem>(GET_LOAN_LIST)
        .bind(&name_debtor)
        .fetch_all(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "No se pudieron obtener los prestamos",
                "get_loans_by_debtor_name",
            )
        })
}

/// Return list of payments made to a loan
pub async fn get_payments_loan(connection: &PgPool, loan_id: Uuid) -> AppResult<Vec<LoanPayment>> {
    sqlx::query_as::<_, LoanPayment>(GET_PAYMENTS_LOAN)
        .bind(loan_id)
        .fetch_all(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "No se pudieron obtener los pagos del prestamo",
                "get_payments_loan",
            )
        })
}

/// Insert a new payment to a loan
pub async fn add_new_payment_loan(
    connection: &PgPool,
    loan: Uuid,
    payment: PgMoney,
) -> AppResult<()> {
    sqlx::query(INSERT_NEW_PAYMENT_LOAN)
        .bind(loan)
        .bind(payment)
        .execute(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "No se pudieron obtener los pagos del prestamo",
                "get_payments_loan",
            )
        })
        .map(|_| ())
}

/// Get total stats loans
pub async fn get_total_loans(
    connection: &PgPool,
    start_date: String,
    end_date: String,
) -> AppResult<TotalLoans> {
    sqlx::query_as::<_, TotalLoans>(GET_LOAN_TOTAL)
        .bind(start_date)
        .bind(end_date)
        .fetch_one(connection)
        .await
        .map_err(|err| {
            get_db_error(
                &err.to_string(),
                "No se pudieron obtener el total de los prestamos",
                "get_total_loans",
            )
        })
}
