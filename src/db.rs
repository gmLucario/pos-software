use once_cell::sync::OnceCell;
use sqlx::{pool::Pool, postgres::Postgres};

#[derive(Debug)]
pub struct Db {
    pub connection: Pool<Postgres>,
}

pub static INSTANCE_DB: OnceCell<Db> = OnceCell::new();

impl Db {
    pub fn global() -> &'static Self {
        INSTANCE_DB.get().expect("logger is not initialized")
    }

    pub fn with_pool_connection(pool: Pool<Postgres>) -> Self {
        Self { connection: pool }
    }
}
