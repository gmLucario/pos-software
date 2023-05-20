pub mod queries;

use once_cell::sync::OnceCell;
use sqlx::postgres::PgPool;

pub struct AppDb {
    pub connection: PgPool,
}

pub static INSTANCE: OnceCell<AppDb> = OnceCell::new();

impl AppDb {
    pub fn get() -> &'static AppDb {
        INSTANCE.get().expect("app pool not set")
    }

    pub fn with_pool_connection(pool: PgPool) -> Self {
        Self { connection: pool }
    }
}
