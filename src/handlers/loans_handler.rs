//! Loans Handler
//!
//! UI event handlers for loan management and payment processing.

use crate::api::{LoanStats, LoanWithPayments, LoansApi};
use crate::models::{Loan, LoanInput, LoanPaymentInput};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct LoansHandler {
    api: Arc<LoansApi>,
}

impl LoansHandler {
    pub fn new(api: Arc<LoansApi>) -> Self {
        Self { api }
    }

    /// Create a loan from a sale
    pub async fn create_loan(&self, sale_id: String, input: LoanInput) -> Result<Loan, String> {
        self.api.create_loan(sale_id, input).await
    }

    /// Get loan details with payment history
    pub async fn get_loan_details(&self, id: String) -> Result<LoanWithPayments, String> {
        self.api.get_loan_with_payments(&id).await
    }

    /// Load all loans
    pub async fn load_loans(&self) -> Result<Vec<Loan>, String> {
        self.api.list_loans().await
    }

    /// Get active loans only
    pub async fn get_active_loans(&self) -> Result<Vec<Loan>, String> {
        self.api.get_active_loans().await
    }

    /// Get loans by status
    pub async fn get_loans_by_status(&self, status_id: i32) -> Result<Vec<Loan>, String> {
        self.api.get_loans_by_status(status_id).await
    }

    /// Search loans by debtor name or phone
    pub async fn search_loans(&self, query: String) -> Result<Vec<Loan>, String> {
        self.api.search_loans(&query).await
    }

    /// Record a payment for a loan
    pub async fn record_payment(&self, input: LoanPaymentInput) -> Result<Loan, String> {
        // Record the payment
        self.api.record_payment(input.clone()).await?;

        // Return updated loan
        self.api.get_loan(&input.loan_id).await
    }

    /// Cancel a loan
    pub async fn cancel_loan(&self, id: String) -> Result<(), String> {
        self.api.cancel_loan(&id).await
    }

    /// Get loan statistics
    pub async fn get_loan_stats(&self) -> Result<LoanStats, String> {
        self.api.get_loan_stats().await
    }

    /// Get overdue loans
    pub async fn get_overdue_loans(&self) -> Result<Vec<Loan>, String> {
        self.api.get_overdue_loans().await
    }
}
