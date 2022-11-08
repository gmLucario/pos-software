//! Database connection instance

use once_cell::sync::OnceCell;
use sqlx::{pool::Pool, postgres::Postgres};

/// Represents database components
#[derive(Debug)]
pub struct Db {
    /// Pool database connection
    pub connection: Pool<Postgres>,
}

/// Singleton instance of [`crate::db::Db`]
pub static INSTANCE_DB: OnceCell<Db> = OnceCell::new();

impl Db {
    /// Get the [`crate::db::Db`] instance previously initialized
    pub fn global() -> Result<&'static Self, String> {
        let instance = INSTANCE_DB.get().ok_or("db struct is not initialized")?;
        Ok(instance)
    }

    /// Initialized the [`crate::db::Db`] struct with a provided `pool`
    pub fn with_pool_connection(pool: Pool<Postgres>) -> Self {
        Self { connection: pool }
    }
}
