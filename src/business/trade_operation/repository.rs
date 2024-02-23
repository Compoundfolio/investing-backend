
use diesel::{insert_into, prelude::*, upsert::excluded};
use uuid::Uuid;

use crate::database::{schema, CommonRepository, RepositoryError};

use super::model::{InsertTradeOperation, SelectTradeOperation};

impl CommonRepository {
    pub fn list_trade_operations(&self, portfolio_id: Uuid) -> Result<Vec<SelectTradeOperation>, RepositoryError> {
        use schema::trade_operation::dsl;
        Ok(dsl::trade_operation
            .filter(dsl::portfolio_id.eq(portfolio_id))
            .select(SelectTradeOperation::as_select())
            .load(&mut self.pool.get()?)?)
    }

    pub fn create_trade_operation(&self, trade_operation: InsertTradeOperation) -> Result<Uuid, RepositoryError> {
        use schema::trade_operation::dsl;
        Ok(diesel::insert_into(dsl::trade_operation)
            .values(trade_operation)
            .returning(dsl::id)
            .get_result::<Uuid>(&mut self.pool.get()?)?)
    }

    pub fn create_trade_operations(&self, trade_operations: Vec<InsertTradeOperation>) -> Result<usize, RepositoryError> {
        use schema::trade_operation::dsl;
        Ok(insert_into(schema::trade_operation::dsl::trade_operation)
            .values(trade_operations)
            .on_conflict((dsl::operation_source, dsl::external_id))
            .do_update()
            .set((
                dsl::report_upload_id.eq(excluded(dsl::report_upload_id)),
                dsl::operation_source.eq(excluded(dsl::operation_source)),
                dsl::external_id.eq(excluded(dsl::external_id)),
                dsl::date_time.eq(excluded(dsl::date_time)),
                dsl::side.eq(excluded(dsl::side)),
                dsl::instrument_symbol.eq(excluded(dsl::instrument_symbol)),
                dsl::isin.eq(excluded(dsl::isin)),
                dsl::price.eq(excluded(dsl::price)),
                dsl::quantity.eq(excluded(dsl::quantity)),
                dsl::commission.eq(excluded(dsl::commission)),
                dsl::order_id.eq(excluded(dsl::order_id)),
                dsl::summ.eq(excluded(dsl::summ)),
                dsl::metadata.eq(excluded(dsl::metadata)),
            ))
            .execute(&mut self.pool.get()?)?)
    }
}
