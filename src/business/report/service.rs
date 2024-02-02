use uuid::Uuid;

use crate::{ApplicationState, business::report::model::{InsertReportUpload, InsertTransaction, InsertTradeOperation}, auth::repository, web::graphql::DescriptiveError};

use super::model::{BrokerType, AbstractReport, ReportProcessingError, ReportProcessingResult, AbstractTransactionType};

pub async fn process_report<R: tokio::io::AsyncRead + Unpin>(
    state: &ApplicationState,
    portfolio_id: Uuid,
    broker: BrokerType,
    reader: R,
    original_filename: String
) -> Result<ReportProcessingResult, DescriptiveError> {
    let parsed: Result<AbstractReport, ReportProcessingError> = match broker {
        BrokerType::Exante => super::exante::parse::parse_report(reader)
            .await
            .map(|ok| ok.into())
            .map_err(|err| err.into()),
        BrokerType::Freedomfinance => super::freedomfinance::parse::parse_report(reader)
            .await
            .map(|ok| ok.into())
            .map_err(|err| err.into()),
    };
    let AbstractReport { transactions, trade_operations, .. } = parsed?;


    for each in transactions.iter() {
        if let AbstractTransactionType::Unrecognized(variant) = &each.operation_type {
           tracing::warn!("When parsing {broker} report for transactions, found unrecognized type: '{variant}'");
        }
    }

    let report_upload_id = state.repository.create_report_upload(InsertReportUpload { 
        portfolio_id, label: original_filename, broker
    })?;

    let inserted_transactions = state.repository.create_transactions(
        transactions.into_iter().map(|t| InsertTransaction {
            portfolio_id,
            report_upload_id,
            transaction: t
        }).collect()
    )?;

    let inserted_trade_opertaions = state.repository.create_trade_operations(
        trade_operations.into_iter().map(|to| InsertTradeOperation {
            portfolio_id,
            report_upload_id,
            trade_operation: to,
        }).collect()
    )?;

    Ok(ReportProcessingResult {
        id: report_upload_id,
        transactions: inserted_transactions,
        trade_operations: inserted_trade_opertaions,
    })
}
