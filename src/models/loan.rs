//! Loan Models
//!
//! Represents customer debt tracking and payment history.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Loan entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Loan {
    pub id: String,  // References sale.id

    #[sqlx(try_from = "String")]
    pub total_debt: Decimal,

    #[sqlx(try_from = "String")]
    pub paid_amount: Decimal,

    #[sqlx(try_from = "String")]
    pub remaining_amount: Decimal,

    pub debtor_name: String,
    pub debtor_phone: Option<String>,
    pub status_id: i32,

    #[sqlx(try_from = "String")]
    pub created_at: DateTime<Utc>,
}

impl Loan {
    /// Check if loan is fully paid off
    pub fn is_paid_off(&self) -> bool {
        self.remaining_amount <= Decimal::ZERO
    }

    /// Calculate payment percentage
    pub fn payment_percentage(&self) -> f64 {
        if self.total_debt > Decimal::ZERO {
            let percentage = (self.paid_amount / self.total_debt) * Decimal::from(100);
            percentage.to_string().parse().unwrap_or(0.0)
        } else {
            0.0
        }
    }

    /// Update amounts after a payment
    pub fn apply_payment(&mut self, amount: Decimal) {
        self.paid_amount += amount;
        self.remaining_amount = if self.total_debt > self.paid_amount {
            self.total_debt - self.paid_amount
        } else {
            Decimal::ZERO
        };
    }
}

/// Loan payment entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LoanPayment {
    pub id: String,  // UUID as TEXT
    pub loan_id: String,

    #[sqlx(try_from = "String")]
    pub amount: Decimal,

    #[sqlx(try_from = "String")]
    pub payment_date: DateTime<Utc>,

    pub notes: Option<String>,
}

/// Input for creating a new loan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanInput {
    pub sale_id: String,
    pub debtor_name: String,
    pub debtor_phone: Option<String>,
}

impl LoanInput {
    /// Create a Loan from Sale info
    pub fn to_loan(&self, total_debt: Decimal, paid_amount: Decimal) -> Loan {
        Loan {
            id: self.sale_id.clone(),  // Loan ID = Sale ID
            total_debt,
            paid_amount,
            remaining_amount: total_debt - paid_amount,
            debtor_name: self.debtor_name.clone(),
            debtor_phone: self.debtor_phone.clone(),
            status_id: if paid_amount >= total_debt {
                3  // Fully Paid
            } else if paid_amount > Decimal::ZERO {
                2  // Partially Paid
            } else {
                1  // Active
            },
            created_at: Utc::now(),
        }
    }
}

/// Input for recording a loan payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanPaymentInput {
    pub loan_id: String,
    pub amount: Decimal,
    pub notes: Option<String>,
}

impl LoanPaymentInput {
    /// Convert to LoanPayment entity
    pub fn to_payment(&self) -> LoanPayment {
        LoanPayment {
            id: uuid::Uuid::new_v4().to_string(),
            loan_id: self.loan_id.clone(),
            amount: self.amount,
            payment_date: Utc::now(),
            notes: self.notes.clone(),
        }
    }
}
