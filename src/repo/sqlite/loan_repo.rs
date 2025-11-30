//! SQLite Loan Repository Implementation

use async_trait::async_trait;
use sqlx::SqlitePool;
use rust_decimal::Decimal;
use crate::models::{Loan, LoanPayment, LoanInput, LoanPaymentInput};
use crate::repo::LoanRepository;

pub struct SqliteLoanRepository {
    pool: SqlitePool,
}

impl SqliteLoanRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LoanRepository for SqliteLoanRepository {
    async fn create(&self, input: LoanInput, total_debt: Decimal, paid_amount: Decimal) -> Result<Loan, String> {
        let loan = input.to_loan(total_debt, paid_amount);

        sqlx::query(
            r#"
            INSERT INTO loan (
                id, total_debt, paid_amount, remaining_amount,
                debtor_name, debtor_phone, status_id, created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&loan.id)
        .bind(loan.total_debt.to_string())
        .bind(loan.paid_amount.to_string())
        .bind(loan.remaining_amount.to_string())
        .bind(&loan.debtor_name)
        .bind(&loan.debtor_phone)
        .bind(loan.status_id)
        .bind(loan.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to create loan: {}", e))?;

        Ok(loan)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Loan>, String> {
        let loan = sqlx::query_as::<_, Loan>(
            "SELECT * FROM loan WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to get loan by id: {}", e))?;

        Ok(loan)
    }

    async fn list_all(&self) -> Result<Vec<Loan>, String> {
        let loans = sqlx::query_as::<_, Loan>(
            "SELECT * FROM loan ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to list loans: {}", e))?;

        Ok(loans)
    }

    async fn update_status(&self, id: &str, status_id: i32) -> Result<(), String> {
        sqlx::query(
            "UPDATE loan SET status_id = ? WHERE id = ?"
        )
        .bind(status_id)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to update loan status: {}", e))?;

        Ok(())
    }

    async fn record_payment(&self, input: LoanPaymentInput) -> Result<LoanPayment, String> {
        let payment = input.to_payment();

        // Start transaction
        let mut tx = self.pool.begin()
            .await
            .map_err(|e| format!("Failed to start transaction: {}", e))?;

        // Insert payment
        sqlx::query(
            r#"
            INSERT INTO loan_payment (
                id, loan_id, amount, payment_date, notes
            )
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&payment.id)
        .bind(&payment.loan_id)
        .bind(payment.amount.to_string())
        .bind(payment.payment_date.to_rfc3339())
        .bind(&payment.notes)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Failed to insert payment: {}", e))?;

        // Update loan amounts
        sqlx::query(
            r#"
            UPDATE loan
            SET paid_amount = paid_amount + ?,
                remaining_amount = CASE
                    WHEN total_debt > (paid_amount + ?) THEN total_debt - (paid_amount + ?)
                    ELSE '0'
                END,
                status_id = CASE
                    WHEN total_debt <= (paid_amount + ?) THEN 3  -- Fully Paid
                    WHEN (paid_amount + ?) > '0' THEN 2  -- Partially Paid
                    ELSE 1  -- Active
                END
            WHERE id = ?
            "#,
        )
        .bind(payment.amount.to_string())
        .bind(payment.amount.to_string())
        .bind(payment.amount.to_string())
        .bind(payment.amount.to_string())
        .bind(payment.amount.to_string())
        .bind(&payment.loan_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Failed to update loan amounts: {}", e))?;

        // Commit transaction
        tx.commit()
            .await
            .map_err(|e| format!("Failed to commit transaction: {}", e))?;

        Ok(payment)
    }

    async fn get_payments(&self, loan_id: &str) -> Result<Vec<LoanPayment>, String> {
        let payments = sqlx::query_as::<_, LoanPayment>(
            "SELECT * FROM loan_payment WHERE loan_id = ? ORDER BY payment_date DESC"
        )
        .bind(loan_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to get loan payments: {}", e))?;

        Ok(payments)
    }

    async fn get_active(&self) -> Result<Vec<Loan>, String> {
        let loans = sqlx::query_as::<_, Loan>(
            "SELECT * FROM loan WHERE status_id IN (1, 2) ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to get active loans: {}", e))?;

        Ok(loans)
    }

    async fn get_by_status(&self, status_id: i32) -> Result<Vec<Loan>, String> {
        let loans = sqlx::query_as::<_, Loan>(
            "SELECT * FROM loan WHERE status_id = ? ORDER BY created_at DESC"
        )
        .bind(status_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to get loans by status: {}", e))?;

        Ok(loans)
    }

    async fn search(&self, query: &str) -> Result<Vec<Loan>, String> {
        let search_term = format!("%{}%", query);

        let loans = sqlx::query_as::<_, Loan>(
            r#"
            SELECT * FROM loan
            WHERE debtor_name LIKE ? OR debtor_phone LIKE ?
            ORDER BY created_at DESC
            "#
        )
        .bind(&search_term)
        .bind(&search_term)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to search loans: {}", e))?;

        Ok(loans)
    }
}
