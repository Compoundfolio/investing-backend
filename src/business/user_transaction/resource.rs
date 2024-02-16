
use async_graphql::{Context, SimpleObject, Object};
use serde::Serialize;
use uuid::Uuid;

use crate::business::portfolio::security::is_portfolio_owner;
use crate::business::model::{BrokerType, Money};
use crate::web::graphql::{get_claims, get_state};



#[derive(Default)]
pub struct UserTransactionQuery;
#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl UserTransactionQuery {
    /// List of user transactions in the requested portfolio
    async fn user_transactions<'ctx>(&self, ctx: &Context<'ctx>, portfolio_id: Uuid) -> async_graphql::Result<Vec<UserTransaction>> {
        let claims = get_claims(ctx)?;
        let state = get_state(ctx)?;

        is_portfolio_owner(state, claims.sub, portfolio_id)?;

        let user_transactions = super::service::generate_user_transaction_list(state, portfolio_id)?;
        Ok(user_transactions)
    }
}


#[derive(Default)]
pub struct UserTransactionMutation;
#[Object(rename_fields="camelCase", rename_args="camelCase")]
impl UserTransactionMutation {
    /// Create a new user-transaction
    async fn create_user_transaction(&self, _ctx: &Context<'_> /* , data: CreatePortfolio */ ) -> async_graphql::Result<String> {
        // let claims = get_claims(ctx)?;
        // let state = get_state(ctx)?;
        // example : let created = state.repository.create_portfolio(claims.sub, &data.label)?;
        // example : Ok(created.into())
        Ok("OK".to_owned())
    }

    /// Delete a user transaction
    async fn delete_user_transaction(&self, _ctx: &Context<'_>, _id: Uuid) -> async_graphql::Result<String> {
        // let claims = get_claims(ctx)?;
        // let state = get_state(ctx)?;
        // example : state.repository.delete_portfolio(claims.sub, id)?;
        Ok("OK".to_owned())
    }
}

// --- model



/// Depending on this type, some of the fields in the UserTransaction can 
/// be present or absent. WARNING: Might contain undocumented values.
#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq, Serialize)]
pub enum UserTransactionType {
    Trade,
    /// An unknown type of transaction was recieved from a brokerage report.
    Unrecognized,
    Tax,
    Divident,
    Comission,
    FundingWithdrawal,
    RevertedDivident,
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq, Serialize)]
pub enum UserTransactionTradeSide { Buy, Sell }



/// User-transaction is any operation recorded in a portfolio, like trade
/// or a fiscal transaction, seen from user perspective as a single entity
/// type. In other words, it is an abstraction above trade operations and
/// fiscal transactions.
#[derive(SimpleObject, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTransaction {
    pub user_transaction_type: UserTransactionType,
    /// Optionally contains name of the brokerage that manages the transaction.
    pub brokerage: Option<BrokerType>,
    /// total change of balance as a result of the transaction. Can be negative.
    pub summ: Money,
    /// Ticker of the related instrument. Appears in TRADE, DIVIDENT and sometimes TAX operations.
    pub symbol: Option<String>,
    /// Appears in TRADE. Contains always positive price of a single instrument.
    pub price: Option<Money>,
    /// Appears in TRADE. Contains always positive amount of securities in a trade.
    pub quantity: Option<i32>,
    /// Appears in TRADE.
    pub trade_side: Option<UserTransactionTradeSide>,
    /// When the UserTransacton is derived from a TradeOperation entity, contains its unique ID.
    pub trade_operation_id: Option<Uuid>,
    /// When the UserTransacton is derived from a FiscalTransaction entity, contains its unique ID.
    pub fiscal_transaction_id: Option<Uuid>,
    /// when user_transaction_type == UNRECOGINZED, contains the original transaction type.
    pub unrecognized_type: Option<String>,
    /// Timestamp of an instant when the transaction occurred.
    pub date_time: chrono::prelude::NaiveDateTime,


}
