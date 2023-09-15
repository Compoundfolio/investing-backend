use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;


use super::freedomfinance::model::CashInOutType;
use super::exante::model::TransactionOperationType;


pub enum AbstractOperationSource {
    ExanteReport, FreedomfinanceReport
}

pub enum AbstractTradeSide {
    Buy,
    Sell
}

pub struct AbstractTradeOperation {
    pub source: AbstractOperationSource,
    pub external_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub side: AbstractTradeSide,
    pub instrument_symbol: String,
    pub isin: String,
    pub price: Decimal,
    pub currency: String,
    pub quantity: u32,
    pub commission: Decimal,
    pub commission_currency: String,
    pub order_id: String,
    pub summ: Decimal,
    pub metadata: HashMap<&'static str,String>
}

pub struct AbstractTransaction {
    pub source: AbstractOperationSource,
    pub external_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub symbol_id: Option<String>,
    pub amount: Decimal,
    pub currency: String,
    pub operation_type: AbstractTransactionType,
    pub comission: Option<Decimal>,
    pub comission_currency: Option<String>,
    pub metadata: HashMap<&'static str,String>
}

impl From<super::exante::model::TradeOperation> for AbstractTradeOperation {
    fn from(value: super::exante::model::TradeOperation) -> Self {
        Self { 
            source: AbstractOperationSource::ExanteReport,
            external_id: Some(format!("{}/{}", value.order_id, value.order_pos)),
            timestamp: value.timestamp,
            side: match value.side {
                super::exante::model::TradeOperationSide::Buy => AbstractTradeSide::Buy,
                super::exante::model::TradeOperationSide::Sell => AbstractTradeSide::Sell,
            },
            instrument_symbol: value.symbol_id,
            isin: value.isin,
            price: value.price,
            currency: value.currency,
            quantity: value.quantity,
            commission: value.commission,
            commission_currency: value.commission_currency,
            order_id: value.order_id.to_string(),
            summ: value.traded_volume,
            metadata: HashMap::from([
                ("uti", value.uti),
                ("trade_type", value.trade_type),
                ("type", value.trade_operation_type),
                ("order_pos", value.order_pos.to_string()),
                ("account_id", value.account_id)
            ])
        }
    }
}

impl From<super::freedomfinance::model::DetailedTrade> for AbstractTradeOperation {
    fn from(value: super::freedomfinance::model::DetailedTrade) -> Self {
        Self { 
            source: AbstractOperationSource::FreedomfinanceReport,
            external_id: Some(value.id.to_string()),
            timestamp: value.date,
            side: match value.operation { 
                super::freedomfinance::model::TradeOperationSide::Buy => AbstractTradeSide::Buy,
                super::freedomfinance::model::TradeOperationSide::Sell => AbstractTradeSide::Sell,
            },
            instrument_symbol: value.instr_nm,
            isin: value.isin,
            price: value.price,
            currency: value.curr_c,
            quantity: value.quantity,
            commission: value.commission,
            commission_currency: value.commission_currency,
            order_id: value.order_id.to_string(),
            summ: value.summ,
            metadata: HashMap::from([
                ("comment", value.comment),
                ("market", value.mkt_name),
                ("instr_kind", value.instr_kind),
                ("trade_id", value.trade_id.to_string())
            ])
        }
    }
}
pub enum AbstractTransactionType {
    Tax,
    Divident,
    Trade,
    Commission,
    FundingWithdrawal,
    RevertedDivident,
    Unrecognized(String),
}

impl From<super::exante::model::Transaction> for AbstractTransaction {
    fn from(value: super::exante::model::Transaction) -> Self {
        Self {
            source: AbstractOperationSource::ExanteReport,
            external_id: Some(value.id),
            timestamp: value.timestamp,
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
            metadata: HashMap::from([
                ("account_id", value.account_id),
                ("isin", value.isin),
                ("eur_equivalent", value.eur_equivalent.to_string()),
                ("comment", value.comment.to_string()),
                ("uuid", value.uuid),
                ("parent_uuid", value.parent_uuid),
            ]),
        }
    }
}

impl From<super::freedomfinance::model::CashInOut> for AbstractTransaction {
    fn from(value: super::freedomfinance::model::CashInOut) -> Self {
        Self {
            source: AbstractOperationSource::FreedomfinanceReport,
            external_id: Some(value.id.to_string()),
            timestamp: value.datetime,
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
            metadata: HashMap::from([
                ("transaction_id", value.transaction_id.to_string()),
                ("details", value.details),
                ("value_usd_details", value.value_usd_details),
                ("reverted", value.reverted.to_string()),
            ]),
        }
    }
}


