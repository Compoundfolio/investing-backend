
use crate::database::schema::fiscal_transaction::dsl;
use diesel::{insert_into, prelude::*, upsert::excluded};
use uuid::Uuid;

use crate::database::{CommonRepository, RepositoryError};

use super::model::{InsertFiscalTransaction, SelectFiscalTransaction};

impl CommonRepository {
    pub fn list_fiscal_transactions(&self, portfolio_id: Uuid) -> Result<Vec<SelectFiscalTransaction>, RepositoryError> {
        Ok(dsl::fiscal_transaction
            .filter(dsl::portfolio_id.eq(portfolio_id))
            .select(SelectFiscalTransaction::as_select())
            .load(&mut self.pool.get()?)?)
    }

    pub fn find_fiscal_transaction_by_id(&self, fiscal_transaction_id: Uuid) -> Result<Option<SelectFiscalTransaction>, RepositoryError> {
        Ok(dsl::fiscal_transaction
            .filter(dsl::id.eq(fiscal_transaction_id))
            .select(SelectFiscalTransaction::as_select())
            .first(&mut self.pool.get()?)
            .optional()?)
    }

    pub fn create_fiscal_transaction(&self, fiscal_transaction: InsertFiscalTransaction) -> Result<Uuid, RepositoryError> {
        Ok(diesel::insert_into(dsl::fiscal_transaction)
            .values(fiscal_transaction)
            .returning(dsl::id)
            .get_result::<Uuid>(&mut self.pool.get()?)?)
    }

    pub fn delete_fiscal_transaction(&self, id: Uuid) -> Result<usize, RepositoryError> {
        let affected = diesel::delete(dsl::fiscal_transaction
            .filter(dsl::id.eq(id)))
            .execute(&mut self.pool.get()?)?;
        Ok(affected)
    }

    pub fn create_fiscal_transactions(&self, fiscal_transactions: Vec<InsertFiscalTransaction>) -> Result<usize, RepositoryError> {
        Ok(insert_into(dsl::fiscal_transaction)
            .values(fiscal_transactions)
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
}
