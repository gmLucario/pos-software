//! Repository Module
//!
//! Data access layer implementing the Repository Pattern.
//! Provides abstraction over database operations.

pub mod traits;
pub mod sqlite;

pub use traits::*;
pub use sqlite::*;
