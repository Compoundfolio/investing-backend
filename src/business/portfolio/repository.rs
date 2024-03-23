use diesel::prelude::*;
use uuid::Uuid;

use crate::{business::model::BrokerType, database::{schema::{self, portfolio::dsl}, CommonRepository, RepositoryError}};

use super::model::{SelectPortfolio, InsertPortfolio};

impl CommonRepository {
    pub fn create_portfolio(&self, user_id: Uuid, label: &str) -> Result<SelectPortfolio, RepositoryError> {
        let result: SelectPortfolio = diesel::insert_into(dsl::portfolio)
            .values(InsertPortfolio {
                app_user_id: user_id,
                label
            })
            .returning(SelectPortfolio::as_select())
            .get_result::<SelectPortfolio>(&mut self.pool.get()?)?;
        Ok(result)
    }

    pub fn find_portfolio_by_id(&self, id: Uuid) -> Result<Option<SelectPortfolio>, RepositoryError> {
        Ok(dsl::portfolio
           .find(id)
           .select(SelectPortfolio::as_select())
           .first(&mut self.pool.get()?)
           .optional()?)
    }

    pub fn list_portfolios(&self, user_id: Uuid) -> Result<Vec<SelectPortfolio>, RepositoryError> {
        Ok(dsl::portfolio
            .filter(dsl::app_user_id.eq(user_id))
            .select(SelectPortfolio::as_select())
            .load(&mut self.pool.get()?)?)
    }

    pub fn list_portfolio_brokerages(&self, portfolio_id: Uuid) -> Result<Vec<BrokerType>, RepositoryError> {
        let result: Vec<Option<BrokerType>> = schema::trade_operation::dsl::trade_operation
            .filter(schema::trade_operation::dsl::portfolio_id.eq(portfolio_id))
            .select(schema::trade_operation::dsl::broker)
            .distinct()
            .union(
                schema::fiscal_transaction::dsl::fiscal_transaction
                    .filter(schema::fiscal_transaction::dsl::portfolio_id.eq(portfolio_id))
                    .select(schema::fiscal_transaction::dsl::broker)
                    .distinct()
            )
            .load(&mut self.pool.get()?)?;

        let mut result: Vec<BrokerType> = result.into_iter()
            .flatten()
            .collect();

        result.sort();
        result.dedup();

        Ok(result)
    }

    pub fn delete_portfolio(&self, user_id: Uuid, porfolio_id: Uuid) -> Result<usize, RepositoryError> {
        let affected = diesel::delete(dsl::portfolio
            .filter(dsl::app_user_id.eq(user_id))
            .filter(dsl::id.eq(porfolio_id)))
            .execute(&mut self.pool.get()?)?;
        Ok(affected)
    }
}
