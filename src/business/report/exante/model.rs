use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_enum_str::Deserialize_enum_str;
use uuid::Uuid;

#[allow(unused_imports)]
use crate::util::serde::date_time_format;
use crate::util::serde::deserialize_uuid_or_none;

#[derive(Deserialize_enum_str)]
pub enum TradeOperationSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
    #[serde(other)]
    Unrecognized(String),
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct TradeOperation {
    #[serde(rename = "Time", with = "date_time_format")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "Account ID")]
    pub account_id: String,
    #[serde(rename = "Side")]
    pub side: TradeOperationSide,
    #[serde(rename = "Symbol ID")]
    pub symbol_id: String,
    #[serde(rename = "ISIN")]
    pub isin: String,
    #[serde(rename = "Type")]
    pub trade_operation_type: String,
    #[serde(rename = "Price")]
    pub price: Decimal,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Quantity")]
    pub quantity: i32,
    #[serde(rename = "Commission")]
    pub comission: Decimal,
    #[serde(rename = "Commission Currency")]
    pub comission_currency: String,
    #[serde(rename = "P&L")]
    pub pnl: Decimal,
    #[serde(rename = "Traded Volume")]
    pub traded_volume: Decimal,
    #[serde(rename = "Order Id")]
    pub order_id: Uuid,
    #[serde(rename = "Order pos")]
    pub order_pos: i32,
    #[serde(rename = "Value Date")]
    pub value_date: String,
    #[serde(rename = "Unique Transaction Identifier (UTI)")]
    pub uti: String,
    #[serde(rename = "Trade type")]
    pub trader_type: String,
}

#[derive(Deserialize_enum_str)]
pub enum TransactionOperationType {
    #[serde(rename = "US TAX")]
    UsTax,
    #[serde(rename = "DIVIDENT")]
    Divident,
    #[serde(rename = "TRADE")]
    Trade,
    #[serde(rename = "COMISSION")]
    Comission,
    #[serde(rename = "FUNDING/WITHDRAWAL")]
    FundingWithdrawal,
    #[serde(other)]
    Unrecognized(String),
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Transaction {
    #[serde(rename = "Transaction ID")]
    pub id: String,
    #[serde(rename = "Account ID")]
    pub account_id: String,
    #[serde(rename = "Symbol ID")]
    pub symbol_id: String,
    #[serde(rename = "ISIN")]
    pub isin: String,
    #[serde(rename = "Operation type")]
    pub operation_type: TransactionOperationType,
    #[serde(rename = "When", with = "date_time_format")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "Sum")]
    pub sum: Decimal,
    #[serde(rename = "Asset")]
    pub asset: String,
    #[serde(rename = "EUR equivalent")]
    pub eur_equivalent: Decimal,
    #[serde(rename = "Comment")]
    pub comment: String,
    #[serde(rename = "UUID", deserialize_with = "deserialize_uuid_or_none")]
    pub uuid: Option<Uuid>,
    #[serde(rename = "Parent UUID", deserialize_with = "deserialize_uuid_or_none")]
    pub perant_uuid: Option<Uuid>,
}

pub struct Report {
    pub trade_operations: Vec<TradeOperation>,
    pub transactions: Vec<Transaction>,
}
