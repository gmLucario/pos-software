//! Loans API
//!
//! Business logic for loan management and payment processing.

use crate::models::{Loan, LoanInput, LoanPayment, LoanPaymentInput, StatusLoan};
use crate::repo::{LoanRepository, SaleRepository};
use rust_decimal::Decimal;
use std::sync::Arc;

#[derive(Clone)]
pub struct LoansApi {
    loan_repo: Arc<dyn LoanRepository>,
    sale_repo: Arc<dyn SaleRepository>,
}

impl std::fmt::Debug for LoansApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoansApi").finish()
    }
}

impl PartialEq for LoansApi {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.loan_repo, &other.loan_repo)
            && Arc::ptr_eq(&self.sale_repo, &other.sale_repo)
    }
}

impl LoansApi {
    pub fn new(loan_repo: Arc<dyn LoanRepository>, sale_repo: Arc<dyn SaleRepository>) -> Self {
        Self {
            loan_repo,
            sale_repo,
        }
    }

    /// Create a new loan from a sale
    pub async fn create_loan(&self, sale_id: String, input: LoanInput) -> Result<Loan, String> {
        // Validate the sale exists
        let sale = self
            .sale_repo
            .get_by_id(&sale_id)
            .await?
            .ok_or_else(|| format!("Sale not found: {}", sale_id))?;

        // Validate debtor information
        if input.debtor_name.trim().is_empty() {
            return Err("Debtor name cannot be empty".to_string());
        }

        // Calculate loan amounts from sale
        let total_debt = sale.total_amount;
        let paid_amount = sale.paid_amount;

        if paid_amount >= total_debt {
            return Err("Sale is already fully paid, cannot create loan".to_string());
        }

        // Create the loan with sale ID
        let mut loan_input = input;
        loan_input.sale_id = sale_id;

        self.loan_repo
            .create(loan_input, total_debt, paid_amount)
            .await
    }

    /// Get loan by ID
    pub async fn get_loan(&self, id: &str) -> Result<Loan, String> {
        self.loan_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| format!("Loan not found: {}", id))
    }

    /// Get loan with payment history
    pub async fn get_loan_with_payments(&self, id: &str) -> Result<LoanWithPayments, String> {
        let loan = self.get_loan(id).await?;
        let payments = self.loan_repo.get_payments(id).await?;

        Ok(LoanWithPayments { loan, payments })
    }

    /// List all loans
    pub async fn list_loans(&self) -> Result<Vec<Loan>, String> {
        self.loan_repo.list_all().await
    }

    /// Get active loans (Active or Partially Paid)
    pub async fn get_active_loans(&self) -> Result<Vec<Loan>, String> {
        self.loan_repo.get_active().await
    }

    /// Get loans by status
    pub async fn get_loans_by_status(&self, status_id: i32) -> Result<Vec<Loan>, String> {
        self.loan_repo.get_by_status(status_id).await
    }

    /// Record a payment for a loan
    pub async fn record_payment(&self, input: LoanPaymentInput) -> Result<LoanPayment, String> {
        // Validate loan exists
        let loan = self.get_loan(&input.loan_id).await?;

        // Validate payment amount
        if input.amount <= Decimal::ZERO {
            return Err("Payment amount must be positive".to_string());
        }

        // Check if loan is already paid off
        if loan.status_id == StatusLoan::FULLY_PAID {
            return Err("Loan is already fully paid".to_string());
        }

        // Check if payment exceeds remaining amount
        if input.amount > loan.remaining_amount {
            return Err(format!(
                "Payment amount ${} exceeds remaining debt ${}",
                input.amount, loan.remaining_amount
            ));
        }

        // Record payment (repository handles loan updates)
        self.loan_repo.record_payment(input).await
    }

    /// Search loans by debtor name or phone
    pub async fn search_loans(&self, query: &str) -> Result<Vec<Loan>, String> {
        if query.trim().is_empty() {
            return self.list_loans().await;
        }

        self.loan_repo.search(query).await
    }

    /// Get loan statistics
    pub async fn get_loan_stats(&self) -> Result<LoanStats, String> {
        let loans = self.loan_repo.list_all().await?;
        let active_loans = self.loan_repo.get_active().await?;

        let total_loans = loans.len();
        let active_loan_count = active_loans.len();

        let total_debt = loans.iter().map(|l| l.total_debt).sum();

        let total_paid = loans.iter().map(|l| l.paid_amount).sum();

        let total_remaining = loans.iter().map(|l| l.remaining_amount).sum();

        let fully_paid_count = loans
            .iter()
            .filter(|l| l.status_id == StatusLoan::FULLY_PAID)
            .count();

        Ok(LoanStats {
            total_loans,
            active_loan_count,
            fully_paid_count,
            total_debt,
            total_paid,
            total_remaining,
        })
    }

    /// Mark loan as cancelled
    pub async fn cancel_loan(&self, id: &str) -> Result<(), String> {
        let loan = self.get_loan(id).await?;

        if loan.status_id == StatusLoan::FULLY_PAID {
            return Err("Cannot cancel a fully paid loan".to_string());
        }

        self.loan_repo
            .update_status(id, StatusLoan::CANCELLED)
            .await
    }

    /// Get overdue loans (placeholder - requires due date implementation)
    pub async fn get_overdue_loans(&self) -> Result<Vec<Loan>, String> {
        // For now, just return active loans
        // In future, this would filter by due_date < current_date
        self.get_active_loans().await
    }
}

/// Loan with payment history
#[derive(Debug, Clone)]
pub struct LoanWithPayments {
    pub loan: Loan,
    pub payments: Vec<LoanPayment>,
}

/// Loan statistics
#[derive(Debug, Clone)]
pub struct LoanStats {
    pub total_loans: usize,
    pub active_loan_count: usize,
    pub fully_paid_count: usize,
    pub total_debt: Decimal,
    pub total_paid: Decimal,
    pub total_remaining: Decimal,
}
