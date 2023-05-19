use std::collections::BTreeMap;

use num_traits::FromPrimitive;
use sqlx::{
    postgres::types::PgMoney,
    types::{BigDecimal, Uuid},
};

use crate::{
    constants::PGMONEY_DECIMALS,
    models::{
        loan::{LoanInfo, LoanPayment},
        sale::ProductSale,
    },
};

/// Info related to loans
#[derive(Default)]
pub struct LoanData {
    /// Data to be used to search the loans
    pub debtor_name: String,
    /// money payed by the client
    pub loan_payment: String,
    /// List of loans
    pub loans_by_debtor: BTreeMap<String, LoanInfo>,
    /// Current loan selected
    pub loan_id: Uuid,
    /// Payments selected loan
    pub payments_loan: Vec<LoanPayment>,
}

/// Groups data be render in the `loan` view
#[derive(Default)]
pub struct Loan {
    pub data: LoanData,
    pub sale_products: Vec<ProductSale>,
}

impl Loan {
    // debtor_name trimed and to lower case
    pub fn debtor_name_to_lowercase(&mut self) -> String {
        self.data.debtor_name.trim().to_lowercase()
    }

    /// Return loan payment as [`sqlx::postgres::types::PgMoney`]
    pub fn get_payment_loan(&self) -> PgMoney {
        let payment_amount = self.data.loan_payment.parse::<f64>().unwrap_or_default();
        let payment_amount = BigDecimal::from_f64(payment_amount).unwrap_or_default();

        PgMoney::from_bigdecimal(payment_amount, PGMONEY_DECIMALS).unwrap_or(PgMoney(0))
    }
}
