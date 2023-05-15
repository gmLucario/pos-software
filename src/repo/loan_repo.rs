//! Interaction with the database related with a loan

use sqlx::{postgres::types::PgMoney, types::Uuid, Pool, Postgres};

use crate::{
    models::{
        loan::{LoanItem, LoanPayment, TotalLoans},
        sale::SaleLoan,
    },
    queries::{
        GET_LOAN_LIST, GET_LOAN_TOTAL, GET_PAYMENTS_LOAN, INSERT_NEW_LOAN, INSERT_NEW_PAYMENT_LOAN,
    },
    schemas::loan::LoanSearchSchema,
};

/// Struct to group the functionality related with
/// the interaction of the database and a loan
pub struct LoanRepo {}

impl LoanRepo {
    /// Save a new sale loan
    pub async fn save_new_loan(connection: &Pool<Postgres>, loan: SaleLoan) -> Result<(), String> {
        if !loan.is_valid {
            return Ok(());
        }

        sqlx::query(INSERT_NEW_LOAN)
            .bind(loan.sale_id)
            .bind(loan.money_amount)
            .bind(&loan.name_debtor)
            .execute(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(())
    }

    /// Return the loans that match the data filter
    pub async fn get_loans_user_by_date_range(
        connection: &Pool<Postgres>,
        data: LoanSearchSchema,
    ) -> Result<Vec<LoanItem>, String> {
        let loans = sqlx::query_as::<_, LoanItem>(GET_LOAN_LIST)
            .bind(data.start_date.to_string())
            .bind(data.end_date.to_string())
            .bind(&data.client)
            .fetch_all(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(loans)
    }

    /// Return list of payments made to a loan
    pub async fn get_payments_loan(
        connection: &Pool<Postgres>,
        loan_id: Uuid,
    ) -> Result<Vec<LoanPayment>, String> {
        let payments = sqlx::query_as::<_, LoanPayment>(GET_PAYMENTS_LOAN)
            .bind(loan_id)
            .fetch_all(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(payments)
    }

    /// Insert a new payment to a loan
    pub async fn add_new_payment_loan(
        connection: &Pool<Postgres>,
        loan: Uuid,
        payment: PgMoney,
    ) -> Result<(), String> {
        sqlx::query(INSERT_NEW_PAYMENT_LOAN)
            .bind(loan)
            .bind(payment)
            .execute(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(())
    }

    /// Get total stats loans
    pub async fn get_total_loans(
        connection: &Pool<Postgres>,
        start_date: String,
        end_date: String,
    ) -> Result<TotalLoans, String> {
        let totals: TotalLoans = sqlx::query_as(GET_LOAN_TOTAL)
            .bind(start_date)
            .bind(end_date)
            .fetch_one(connection)
            .await
            .map_err(|err| err.to_string())?;

        Ok(totals)
    }
}
