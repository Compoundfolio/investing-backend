use diesel::{insert_into, prelude::*};

use crate::database::{schema, CommonRepository, RepositoryError};

use super::model::{AbstractTransaction, AbstractTradeOperation};

impl CommonRepository {
    pub fn save_transactions(&self, transactions: Vec<AbstractTransaction>) -> Result<(), RepositoryError> {
        insert_into(schema::transaction::dsl::transaction)
            .values(transactions)
            .execute(&mut self.pool.get()?)?;
        Ok(())
    }

    pub fn save_trade_operations(&self, trade_operations: Vec<AbstractTradeOperation>) -> Result<(), RepositoryError> {
        insert_into(schema::trade_operation::dsl::trade_operation)
            .values(trade_operations)
            .execute(&mut self.pool.get()?)?;
        Ok(())
    }
}
