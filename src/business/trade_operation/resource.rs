
use async_graphql::{Context, CustomValidator, InputObject, InputValueError, Object};
use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;

use crate::business::portfolio::security::is_portfolio_owner;
use crate::business::model::{BrokerType, Money};
use crate::web::errors::DescriptiveError;
use crate::web::graphql::{get_claims, get_state};

use super::model::{InsertTradeOperation, TradeOperation, TradeOperationSide};


#[derive(Default)]
pub struct TradeOperationMutation;
#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl TradeOperationMutation {
    /// Create a new trade opration within a portfolio identified by id
    async fn create_trade_operationg(
        &self, 
        ctx: &Context<'_>, 
        #[graphql(validator(custom = "CreateTradeOperationValidator{}"))]
        create_request: CreateTradeOperation 
    ) -> async_graphql::Result<Uuid> {
        let claims = get_claims(ctx)?;
        let state = get_state(ctx)?;

        is_portfolio_owner(state, claims.sub, create_request.portfolio_id)?;
        let created = state.repository.create_trade_operation(create_request.into())?;
        Ok(created)
    }

    /// Delete a fiscal transaction transaction
    async fn delete_trade_operationg(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<String> {
        let claims = get_claims(ctx)?;
        let state = get_state(ctx)?;
        let to_be_deleted = state.repository.find_trade_operation_by_id(id)?
            .ok_or(DescriptiveError::NotFound { resource: "fiscal transaction".to_owned() })?;
        is_portfolio_owner(state, claims.sub, to_be_deleted.portfolio_id)?;
        state.repository.delete_trade_operation(id)?;
        Ok("OK".to_owned())
    }
}

// --- model

#[derive(InputObject,Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateTradeOperation {
    pub portfolio_id: Uuid,
    /// Ticker of the traded security, instrument symbol.
    pub ticker: String,

    pub side: TradeOperationSide,
    /// Price of a single security in this transaction. Must be positive.
    pub price: Money,
    /// Quantity of traded securities. Must be positive.
    pub quantity: i32,
    /// Required total amount of currency paid for the trade.
    pub summ: Money,
    /// Optionally provide an ISIN identificator.
    pub isin: Option<String>,
    /// Date and time at which this transaction occurred in a local timezone.
    pub date_time: NaiveDateTime,
    /// Associate this transaction with a broker. Not required.
    pub brokerage: Option<BrokerType>,
}


impl From<CreateTradeOperation> for InsertTradeOperation {
    fn from(val: CreateTradeOperation) -> Self {
        InsertTradeOperation {
            portfolio_id: val.portfolio_id,
            report_upload_id: None,
            trade_operation: TradeOperation {
                operation_source: crate::business::model::OperationSource::Manual,
                broker: val.brokerage,
                side: val.side,
                instrument_symbol: val.ticker,
                isin: val.isin,
                price: val.price,
                quantity: val.quantity,
                summ: val.summ,
                date_time: val.date_time,
                order_id: None,
                external_id: None,
                commission: None,
                metadata: serde_json::Value::Null,
            },
        }
    }
}

// --- validation

struct CreateTradeOperationValidator { }

impl CustomValidator<CreateTradeOperation> for CreateTradeOperationValidator {
    fn check(&self, value: &CreateTradeOperation) -> Result<(), InputValueError<CreateTradeOperation>> {
        if value.summ.amount.is_sign_negative() {
            Err(InputValueError::custom("summ.amount must be positive"))
        } else if value.price.amount.is_sign_negative() {
            Err(InputValueError::custom("price.amount must be positive"))
        } else if value.quantity.is_negative()  {
            Err(InputValueError::custom("quantity must be positive"))
        } else {
            Ok(())
        }
    }
}
