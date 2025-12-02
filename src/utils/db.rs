//! Database Utilities
//!
//! Functions for initializing and managing the SQLite database connection.

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::ConnectOptions;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

/// Database connection pool type
pub type DbPool = SqlitePool;

/// Initialize the SQLite database
///
/// Creates the database file if it doesn't exist and runs migrations.
pub async fn initialize_database(database_url: &str) -> Result<DbPool, sqlx::Error> {
    tracing::info!("Initializing database at: {}", database_url);

    // Parse connection options
    let mut options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .foreign_keys(true) // Enable foreign key constraints
        .busy_timeout(Duration::from_secs(30));

    // Disable logging for individual queries (can be noisy)
    options = options.disable_statement_logging();

    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    tracing::info!("Database connection pool created successfully");

    // Run migrations
    run_migrations(&pool).await?;

    Ok(pool)
}

/// Run database migrations
///
/// Executes the SQL schema file to create tables and insert initial data.
async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::Error> {
    tracing::info!("Running database migrations...");

    // Read the migration SQL file
    let migration_sql = include_str!("../../migrations/sqlite_schema.sql");

    // Execute each statement in the migration
    // SQLite doesn't support multiple statements in one execute, so we split them
    for statement in migration_sql.split(';') {
        let statement = statement.trim();
        if !statement.is_empty() {
            sqlx::query(statement).execute(pool).await?;
        }
    }

    tracing::info!("Database migrations completed successfully");

    Ok(())
}

/// Get the default database URL
///
/// Uses the DATA_DIR environment variable or defaults to ./data
pub fn get_database_url() -> String {
    let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string());

    // Create data directory if it doesn't exist
    if let Err(e) = std::fs::create_dir_all(&data_dir) {
        tracing::warn!("Failed to create data directory: {}", e);
    }

    let db_path = Path::new(&data_dir).join("pos.db");
    format!("sqlite:{}", db_path.display())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_initialization() {
        // Use in-memory database for testing
        let db_url = "sqlite::memory:";
        let pool = initialize_database(db_url).await;

        assert!(pool.is_ok(), "Database initialization should succeed");

        let pool = pool.unwrap();

        // Test that we can query the database
        let result = sqlx::query("SELECT COUNT(*) FROM unit_measurement")
            .fetch_one(&pool)
            .await;

        assert!(result.is_ok(), "Should be able to query migrated tables");
    }

    #[test]
    fn test_get_database_url() {
        let url = get_database_url();
        assert!(url.starts_with("sqlite:"));
        assert!(url.contains("pos.db"));
    }
}
