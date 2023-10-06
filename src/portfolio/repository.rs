use diesel::prelude::*;
use uuid::Uuid;

use crate::database::{schema::portfolio::dsl, CommonRepository, RepositoryError};

use super::model::{Portfolio, InsertPortfolio};

impl CommonRepository {
    pub fn create_portfolio(&self, user_id: Uuid, label: &str) -> Result<Portfolio, RepositoryError> {
        let result: Portfolio = diesel::insert_into(dsl::portfolio)
            .values(InsertPortfolio {
                app_user_id: user_id,
                label
            })
            .returning((dsl::id, dsl::label))
            .get_result::<Portfolio>(&mut self.pool.get()?)?;
        Ok(result)
    }

    pub fn list_portfolios(&self, user_id: Uuid) -> Result<Vec<Portfolio>, RepositoryError> {
        Ok(dsl::portfolio
            .filter(dsl::app_user_id.eq(user_id))
            .select(Portfolio::as_select())
            .load(&mut self.pool.get()?)?)
    }

    pub fn delete_portfolio(&self, user_id: Uuid, porfolio_id: Uuid) -> Result<(), RepositoryError> {
        let affected = diesel::delete(dsl::portfolio)
            .filter(dsl::app_user_id.eq(user_id))
            .filter(dsl::id.eq(porfolio_id))
            .execute(&mut self.pool.get()?)?;
        match affected {
            0 => Err(RepositoryError::NoRowsAffected),
            _ => Ok(())
        }
        
    }
}
