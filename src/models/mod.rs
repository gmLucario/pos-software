//! Data Models Module
//!
//! Database entities matching the SQLite schema.

pub mod product;
pub mod sale;
pub mod loan;
pub mod catalogs;

pub use product::{Product, ProductInput};
pub use sale::{Sale, Operation, SaleInput, SaleItemInput};
pub use loan::{Loan, LoanPayment, LoanInput, LoanPaymentInput};
pub use catalogs::{ItemCondition, StatusLoan, UnitMeasurement};
