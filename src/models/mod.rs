//! Data Models Module
//!
//! Database entities matching the SQLite schema.

pub mod product;
pub mod sale;
pub mod loan;
pub mod catalogs;

pub use product::Product;
pub use sale::{Sale, Operation};
pub use loan::{Loan, LoanPayment};
pub use catalogs::{ItemCondition, StatusLoan, UnitMeasurement};
