
use async_graphql::{Context, SimpleObject, Object, Upload, UploadValue};
use tokio_util::compat::FuturesAsyncReadCompatExt;
use serde::Serialize;
use uuid::Uuid;

use crate::business::model::BrokerType;
use crate::business::portfolio::security::is_portfolio_owner;
use crate::web::graphql::{get_claims, get_state};
use super::model::ReportProcessingResult;
use super::service::process_report;



#[derive(Default)]
pub struct ReportMutation;
#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl ReportMutation {
    async fn upload_report(&self, ctx: &Context<'_>, portfolio_id: Uuid, brokerage: BrokerType, upload: Upload) -> async_graphql::Result<ReportUploadResult> {
        let upload_value: UploadValue = upload.value(ctx)?;
        let claims = get_claims(ctx)?;
        let state = get_state(ctx)?;

        is_portfolio_owner(state, claims.sub, portfolio_id)?;

        let original_filename = upload_value.filename.clone();
        let async_read = upload_value.into_async_read();
        let async_read = FuturesAsyncReadCompatExt::compat(async_read);
        let parsed_report = process_report(state, portfolio_id, brokerage, async_read, original_filename).await?;
        Ok(ReportUploadResult::from(parsed_report))
    }
}



// --- model

#[derive(Serialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct ReportUploadResult {
    pub id: Uuid,
    pub fiscal_transactions: usize,
    pub trade_operations: usize
}

impl From<ReportProcessingResult> for ReportUploadResult {
    fn from(value: ReportProcessingResult) -> Self {
        let ReportProcessingResult { id, fiscal_transactions, trade_operations } = value;
        ReportUploadResult { id, fiscal_transactions, trade_operations }
    }
}


