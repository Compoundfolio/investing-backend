use std::sync::Arc;

use async_graphql::{Context, SimpleObject, InputObject, Object, Upload, UploadValue};
use tokio_util::compat::FuturesAsyncReadCompatExt;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::ApplicationState;
use crate::web::routes::graphql::get_claims;

use super::model::{BrokerType, AbstractReport};
use super::service::parse_report;

#[derive(Serialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Portfolio {
    pub id: Uuid,
    pub label: String,
}

#[derive(Default)]
pub struct ReportMutation;
#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl ReportMutation {
    async fn upload_report(&self, ctx: &Context<'_>, brokerage: BrokerType, upload: Upload) -> async_graphql::Result<String> {
        let upload_value: UploadValue = upload.value(ctx)?;
        let async_read = upload_value.into_async_read();
        let async_read = FuturesAsyncReadCompatExt::compat(async_read);
        let parsed_report = parse_report(brokerage, async_read).await?;
        return Ok(format!("Parsed report with {} transactions and {} trade operations", parsed_report.transactions.len(), parsed_report.trade_operations.len()))
    }
}

