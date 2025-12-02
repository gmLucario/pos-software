//! Loan Models
//!
//! Represents customer debt tracking and payment history.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Loan entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Loan {
    pub id: String, // References sale.id

    pub total_debt: Decimal,

    pub paid_amount: Decimal,

    pub remaining_amount: Decimal,

    pub debtor_name: String,
    pub debtor_phone: Option<String>,
    pub status_id: i32,

    pub created_at: DateTime<Utc>,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for Loan {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(Loan {
            id: row.try_get("id")?,
            total_debt: {
                let s: String = row.try_get("total_debt")?;
                Decimal::from_str(&s).map_err(|e| sqlx::Error::ColumnDecode {
                    index: "total_debt".to_string(),
                    source: Box::new(e),
                })?
            },
            paid_amount: {
                let s: String = row.try_get("paid_amount")?;
                Decimal::from_str(&s).map_err(|e| sqlx::Error::ColumnDecode {
                    index: "paid_amount".to_string(),
                    source: Box::new(e),
                })?
            },
            remaining_amount: {
                let s: String = row.try_get("remaining_amount")?;
                Decimal::from_str(&s).map_err(|e| sqlx::Error::ColumnDecode {
                    index: "remaining_amount".to_string(),
                    source: Box::new(e),
                })?
            },
            debtor_name: row.try_get("debtor_name")?,
            debtor_phone: row.try_get("debtor_phone")?,
            status_id: row.try_get("status_id")?,
            created_at: {
                let s: String = row.try_get("created_at")?;
                DateTime::parse_from_rfc3339(&s)
                    .map(|dt| dt.with_timezone(&Utc))
                    .map_err(|e| sqlx::Error::ColumnDecode {
                        index: "created_at".to_string(),
                        source: Box::new(e),
                    })?
            },
        })
    }
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoanPayment {
    pub id: String, // UUID as TEXT
    pub loan_id: String,

    pub amount: Decimal,

    pub payment_date: DateTime<Utc>,

    pub notes: Option<String>,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for LoanPayment {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(LoanPayment {
            id: row.try_get("id")?,
            loan_id: row.try_get("loan_id")?,
            amount: {
                let s: String = row.try_get("amount")?;
                Decimal::from_str(&s).map_err(|e| sqlx::Error::ColumnDecode {
                    index: "amount".to_string(),
                    source: Box::new(e),
                })?
            },
            payment_date: {
                let s: String = row.try_get("payment_date")?;
                DateTime::parse_from_rfc3339(&s)
                    .map(|dt| dt.with_timezone(&Utc))
                    .map_err(|e| sqlx::Error::ColumnDecode {
                        index: "payment_date".to_string(),
                        source: Box::new(e),
                    })?
            },
            notes: row.try_get("notes")?,
        })
    }
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
            id: self.sale_id.clone(), // Loan ID = Sale ID
            total_debt,
            paid_amount,
            remaining_amount: total_debt - paid_amount,
            debtor_name: self.debtor_name.clone(),
            debtor_phone: self.debtor_phone.clone(),
            status_id: if paid_amount >= total_debt {
                3 // Fully Paid
            } else if paid_amount > Decimal::ZERO {
                2 // Partially Paid
            } else {
                1 // Active
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
