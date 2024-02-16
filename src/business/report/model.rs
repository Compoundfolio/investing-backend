


use diesel::{Insertable};


use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::{business::{fiscal_transaction::model::FiscalTransaction, model::BrokerType, trade_operation::model::TradeOperation}, database::schema::{self}};


#[derive(Serialize)]
pub struct AbstractReport {
    pub broker: BrokerType,
    pub trade_operations: Vec<TradeOperation>,
    pub fiscal_transactions: Vec<FiscalTransaction>
}


pub struct ReportProcessingResult {
    pub id: Uuid,
    pub fiscal_transactions: usize,
    pub trade_operations: usize
}

#[derive(thiserror::Error, Debug)]
pub enum ReportProcessingError {
    #[error(transparent)]
    ExanteReportParsingError { #[from] source: super::exante::model::ExanteReportParsingError },
    #[error(transparent)]
    FreedomfinanceReportParsingError { #[from] source: super::freedomfinance::model::FreedomfinanceReportParsingError },
}

// --- orm model

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::report_upload )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertReportUpload {
    pub portfolio_id: Uuid,
    pub label: String,
    pub broker: BrokerType
}

