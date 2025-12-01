//! Data Models Module
//!
//! Database entities matching the SQLite schema.

pub mod catalogs;
pub mod loan;
pub mod product;
pub mod sale;

pub use catalogs::{ItemCondition, StatusLoan, UnitMeasurement};
pub use loan::{Loan, LoanInput, LoanPayment, LoanPaymentInput};
pub use product::{Product, ProductInput};
pub use sale::{Operation, Sale, SaleInput, SaleItemInput};
