pub mod auth;



use r2d2;
use thiserror::Error;

use diesel::pg::PgConnection;

use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database connection pool error")]
    ConnectionPool {
        #[from]
        source: r2d2::Error,
    },
    #[error("Database ORM error")]
    Diesel {
        #[from]
        source: diesel::result::Error,
    },
}

pub struct CommonRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl CommonRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}
