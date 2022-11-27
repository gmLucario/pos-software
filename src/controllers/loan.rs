use sqlx::{
    postgres::types::PgMoney,
    types::{BigDecimal, Uuid},
};
use std::str::FromStr;

use crate::{
    constants::PGMONEY_DECIMALS,
    kinds::LoanModal,
    models::{
        loan::{LoanItem, LoanPayment},
        sale::ProductSale,
    },
    schemas::loan::{LoanSearchSchema, LoanWidgetsStates},
};

/// Info related to loans
#[derive(Default)]
pub struct LoanData {
    /// Data to be used to search the loans
    pub search_info: LoanSearchSchema,
    /// money payed by the client
    pub loan_payment: String,
    /// states to show the date pickers
    pub widgets_states: LoanWidgetsStates,
    /// List of loans
    pub loans: Vec<LoanItem>,
    /// Current loan selected
    pub loan_id: Uuid,
    /// Payments selected loan
    pub payments_loan: Vec<LoanPayment>,
}

/// Groups data be render in the `loan` view
#[derive(Default)]
pub struct Loan {
    pub modal_show: LoanModal,
    pub data: LoanData,
    pub sale_products: Vec<ProductSale>,
}

impl Loan {
    /// Get the loan search data
    pub fn get_loan_search(&self) -> LoanSearchSchema {
        let mut data = self.data.search_info.clone();
        data.client = data
            .client
            .to_lowercase()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");

        data
    }

    /// Set default data to loan data
    pub fn reset_loan_data(&mut self) {
        self.data = LoanData::default();
    }

    /// Set which modal need to be shown
    pub fn set_modal_show(&mut self, modal: LoanModal) {
        self.modal_show = modal;
        self.data.widgets_states.show_modal = true;
    }

    /// Set modal as hidden
    pub fn hide_modal(&mut self) {
        self.data.widgets_states.show_modal = false;
    }

    /// Return loan payment as [`sqlx::postgres::types::PgMoney`]
    pub fn get_payment_loan(&self) -> PgMoney {
        let payment_amount = self.data.loan_payment.parse::<f64>().unwrap();
        let payment_amount = BigDecimal::from_str(&format!("{:.2}", payment_amount)).unwrap();

        PgMoney::from_bigdecimal(payment_amount, PGMONEY_DECIMALS).unwrap()
    }
}
