use chrono::NaiveDateTime;
use diesel::Insertable;
use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};
use serde_json::json;
use serde_enum_str::{Deserialize_enum_str,Serialize_enum_str};

use crate::datasource::diesel::model::report::Money;

use super::freedomfinance::model::CashInOutType;
use super::exante::model::TransactionOperationType;


#[derive(Deserialize_enum_str, Serialize_enum_str)]
#[derive(diesel_derive_enum::DbEnum, Debug)]
#[ExistingTypePath = "crate::datasource::diesel::schema::sql_types::OperationSourceType"]
pub enum AbstractOperationSource {
    ExanteReport, FreedomfinanceReport
}

#[derive(Deserialize_enum_str, Serialize_enum_str)]
#[derive(diesel_derive_enum::DbEnum, Debug)]
#[ExistingTypePath = "crate::datasource::diesel::schema::sql_types::TradeSideType"]
pub enum AbstractTradeSide {
    Buy,
    Sell
}

#[derive(Serialize,Deserialize,Insertable)]
#[diesel(table_name = crate::datasource::diesel::schema::trade_operation )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AbstractTradeOperation {
    pub operation_source: AbstractOperationSource,
    pub external_id: Option<String>,
    pub date_time: NaiveDateTime,
    pub side: AbstractTradeSide,
    pub instrument_symbol: String,
    pub isin: String,
    pub price: Money,
    pub quantity: i32,
    pub commission: Money,
    pub order_id: String,
    pub summ: Money,
    pub metadata: serde_json::Value,
}

#[derive(Serialize,Deserialize)]
pub struct AbstractTransaction {
    pub source: AbstractOperationSource,
    pub external_id: Option<String>,
    pub date_time: NaiveDateTime,
    pub symbol_id: Option<String>,
    pub amount: BigDecimal,
    pub currency: String,
    pub operation_type: AbstractTransactionType,
    pub comission: Option<BigDecimal>,
    pub comission_currency: Option<String>,
    pub metadata: serde_json::Value,
}

#[derive(Deserialize_enum_str, Serialize_enum_str)]
pub enum AbstractTransactionType {
    Tax,
    Divident,
    Trade,
    Commission,
    FundingWithdrawal,
    RevertedDivident,
    #[serde(other)]
    Unrecognized(String),
}

impl From<super::exante::model::TradeOperation> for AbstractTradeOperation {
    fn from(value: super::exante::model::TradeOperation) -> Self {
        Self { 
            operation_source: AbstractOperationSource::ExanteReport,
            external_id: Some(format!("{}/{}", value.order_id, value.order_pos)),
            date_time: value.timestamp,
            side: match value.side {
                super::exante::model::TradeOperationSide::Buy => AbstractTradeSide::Buy,
                super::exante::model::TradeOperationSide::Sell => AbstractTradeSide::Sell,
            },
            instrument_symbol: value.symbol_id,
            isin: value.isin,
            price: Money::new(value.price, value.currency.clone()),
            quantity: value.quantity,
            commission: Money::new(value.commission, value.commission_currency),
            order_id: value.order_id.to_string(),
            summ: Money::new(value.traded_volume, value.currency),
            metadata: json!({
                "uti": value.uti,
                "trade_type": value.trade_type,
                "type": value.trade_operation_type,
                "order_pos": value.order_pos.to_string(),
                "account_id": value.account_id,
            })
        }
    }
}

impl From<super::freedomfinance::model::DetailedTrade> for AbstractTradeOperation {
    fn from(value: super::freedomfinance::model::DetailedTrade) -> Self {
        Self { 
            operation_source: AbstractOperationSource::FreedomfinanceReport,
            external_id: Some(value.id.to_string()),
            date_time: value.date,
            side: match value.operation { 
                super::freedomfinance::model::TradeOperationSide::Buy => AbstractTradeSide::Buy,
                super::freedomfinance::model::TradeOperationSide::Sell => AbstractTradeSide::Sell,
            },
            instrument_symbol: value.instr_nm,
            isin: value.isin,
            price: Money::new(value.price, value.curr_c.clone()),
            quantity: value.quantity,
            commission: Money::new(value.commission, value.commission_currency),
            order_id: value.order_id.to_string(),
            summ: Money::new(value.summ, value.curr_c),
            metadata: json!({
                "comment": value.comment,
                "market": value.mkt_name,
                "instr_kind": value.instr_kind,
                "trade_id": value.trade_id.to_string(),
            })
        }
    }
}

impl From<super::exante::model::Transaction> for AbstractTransaction {
    fn from(value: super::exante::model::Transaction) -> Self {
        Self {
            source: AbstractOperationSource::ExanteReport,
            external_id: Some(value.id),
            date_time: value.timestamp,
            symbol_id: match value.symbol_id.as_str() {
                "None" => None,
                some => Some(some.to_string())
            },
            operation_type: match value.operation_type {
                TransactionOperationType::UsTax => AbstractTransactionType::Tax,
                TransactionOperationType::Divident => AbstractTransactionType::Divident,
                TransactionOperationType::Trade => AbstractTransactionType::Trade,
                TransactionOperationType::Commission => AbstractTransactionType::Commission,
                TransactionOperationType::FundingWithdrawal => AbstractTransactionType::FundingWithdrawal,
                TransactionOperationType::Unrecognized(a) => AbstractTransactionType::Unrecognized(a),
            },
            amount: value.sum,
            currency: value.asset,
            comission: None,
            comission_currency: None,
            metadata: json!({
                "account_id": value.account_id,
                "isin": value.isin,
                "eur_equivalent": value.eur_equivalent.to_string(),
                "comment": value.comment.to_string(),
                "uuid": value.uuid,
                "parent_uuid": value.parent_uuid
            }),
        }
    }
}

impl From<super::freedomfinance::model::CashInOut> for AbstractTransaction {
    fn from(value: super::freedomfinance::model::CashInOut) -> Self {
        Self {
            source: AbstractOperationSource::FreedomfinanceReport,
            external_id: Some(value.id.to_string()),
            date_time: value.datetime,
            symbol_id: value.ticker,
            amount: value.amount,
            currency: value.currency,
            operation_type: match value.operation_type {
                CashInOutType::DividentReverted => AbstractTransactionType::RevertedDivident,
                CashInOutType::Divident => AbstractTransactionType::Divident,
                CashInOutType::Card => AbstractTransactionType::FundingWithdrawal,
                CashInOutType::Unrecognized(a) => AbstractTransactionType::Unrecognized(a),
            },
            comission: match value.commission {
                some if some.to_string() == "0"  => None,
                some => Some(some)
            },
            comission_currency: value.commission_currency,
            metadata: json!({
                "transaction_id": value.transaction_id.to_string(),
                "details": value.details,
                "value_usd_details": value.value_usd_details,
                "reverted": value.reverted.to_string(),
            }),
        }
    }
}


