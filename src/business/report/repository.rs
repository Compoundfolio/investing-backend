use diesel::{insert_into, prelude::*, upsert::excluded};
use uuid::Uuid;

use crate::database::{schema, CommonRepository, RepositoryError};

use super::model::{InsertTransaction, InsertTradeOperation, InsertReportUpload};

impl CommonRepository {

    pub fn create_report_upload(&self, report_upload: InsertReportUpload) -> Result<Uuid, RepositoryError> {
        use schema::report_upload::dsl;
        let report_upload_id = insert_into(dsl::report_upload)
            .values(report_upload)
            .returning(dsl::id)
            .get_result::<Uuid>(&mut self.pool.get()?)?;
        Ok(report_upload_id)
    }

    pub fn create_transactions(&self, transactions: Vec<InsertTransaction>) -> Result<usize, RepositoryError> {
        use schema::transaction::dsl;
        Ok(insert_into(dsl::transaction)
            .values(transactions)
            .on_conflict((dsl::operation_source, dsl::external_id))
            .do_update()
            .set((
                dsl::report_upload_id.eq(excluded(dsl::report_upload_id)),
                dsl::operation_source.eq(excluded(dsl::operation_source)),
                dsl::external_id.eq(excluded(dsl::external_id)),
                dsl::date_time.eq(excluded(dsl::date_time)),
                dsl::symbol_id.eq(excluded(dsl::symbol_id)),
                dsl::amount.eq(excluded(dsl::amount)),
                dsl::operation_type.eq(excluded(dsl::operation_type)),
                dsl::commission.eq(excluded(dsl::commission)),
                dsl::metadata.eq(excluded(dsl::metadata))
            ))
            .execute(&mut self.pool.get()?)?)
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
