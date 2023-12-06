pub mod schema;


use thiserror::Error;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database connection pool error: {source}")]
    ConnectionPool {
        #[from]
        source: r2d2::Error,
    },
    #[error("Database ORM error: {source}")]
    Diesel {
        #[from]
        source: diesel::result::Error,
    },
    #[error("Database update was cancelled, no data was updated")]
    NoRowsAffected
}

pub struct CommonRepository {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl CommonRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}
