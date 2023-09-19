use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use serde::Deserialize;
use serde_enum_str::Deserialize_enum_str;
use uuid::Uuid;

#[allow(unused_imports)]
use crate::util::serde::date_time_format;

#[derive(Deserialize_enum_str)]
pub enum TradeOperationSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell
}

#[derive(Deserialize)]
pub struct TradeOperation {
    #[serde(rename = "Time", with = "date_time_format")]
    pub timestamp: NaiveDateTime,
    #[serde(rename = "Account ID")]
    pub account_id: String,
    #[serde(rename = "Side")]
    pub side: TradeOperationSide,
    #[serde(rename = "Symbol ID")]
    pub symbol_id: String,
    #[serde(rename = "ISIN")]
    pub isin: String,
    #[serde(rename = "Type")]
    pub trade_operation_type: String, // like "STOCK"
    #[serde(rename = "Price")]
    pub price: BigDecimal,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Quantity")]
    pub quantity: i32,
    #[serde(rename = "Commission")]
    pub commission: BigDecimal,
    #[serde(rename = "Commission Currency")]
    pub commission_currency: String,
    #[serde(rename = "P&L")]
    pub pnl: BigDecimal,
    #[serde(rename = "Traded Volume")]
    pub traded_volume: BigDecimal, // as a summ without commission
    #[serde(rename = "Order Id")]
    pub order_id: Uuid,
    #[serde(rename = "Order pos")]
    pub order_pos: i32,
    #[serde(rename = "Value Date")]
    pub value_date: String,
    #[serde(rename = "Unique Transaction Identifier (UTI)")]
    pub uti: String,
    #[serde(rename = "Trade type")]
    pub trade_type: String, // like "TRADE"
}

#[derive(Deserialize_enum_str)]
pub enum TransactionOperationType {
    #[serde(rename = "US TAX")]
    UsTax,
    #[serde(rename = "DIVIDENT")]
    Divident,
    #[serde(rename = "TRADE")]
    Trade,
    #[serde(rename = "COMMISSION")]
    Commission,
    #[serde(rename = "FUNDING/WITHDRAWAL")]
    FundingWithdrawal,
    #[serde(other)]
    Unrecognized(String),
}

#[derive(Deserialize)]
pub struct Transaction {
    #[serde(rename = "Transaction ID")]
    pub id: String,
    #[serde(rename = "Account ID")]
    pub account_id: String,
    #[serde(rename = "Symbol ID")]
    pub symbol_id: String, // ticker or "None"
    #[serde(rename = "ISIN")]
    pub isin: String,
    #[serde(rename = "Operation type")]
    pub operation_type: TransactionOperationType,
    #[serde(rename = "When", with = "date_time_format")]
    pub timestamp: NaiveDateTime,
    #[serde(rename = "Sum")]
    pub sum: BigDecimal,
    #[serde(rename = "Asset")]
    pub asset: String, // always currency or ticker
    #[serde(rename = "EUR equivalent")]
    pub eur_equivalent: BigDecimal,
    #[serde(rename = "Comment")]
    pub comment: String,
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "Parent UUID")]
    pub parent_uuid: String,
}

pub struct Report {
    pub trade_operations: Vec<TradeOperation>,
    pub transactions: Vec<Transaction>,
}
