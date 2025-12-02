//! Repository Module
//!
//! Data access layer implementing the Repository Pattern.
//! Provides abstraction over database operations.

pub mod sqlite;
pub mod traits;

pub use sqlite::*;
pub use traits::*;
