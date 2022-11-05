use sqlx::{Pool, Postgres};

use crate::{models::sale::SaleLoan, queries::INSERT_NEW_LOAN};

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
}
