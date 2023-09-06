use chrono::NaiveDate;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_enum_str::Deserialize_enum_str;

#[allow(unused_imports)]
use crate::util::serde::date_time_format;


#[derive(Deserialize)]
pub struct Report {
    pub trades: Trades,
    pub cash_flows: CashFlows,
}

#[derive(Deserialize)]
pub struct Trades {
    pub detailed: Vec<DetailedTrade>
}

#[derive(Deserialize)]
pub struct CashFlows {
    pub detailed: Vec<CashFlow>
}

#[derive(Deserialize_enum_str)]
pub enum TradeOperationSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
    #[serde(other)]
    Unrecognized(String),
}

#[derive(Deserialize)]
pub struct DetailedTrade {
    pub trade_id: u64,
    #[serde(with = "date_time_format")]
    pub date: DateTime<Utc>,
    pub instr_nm: String,
    pub operation: TradeOperationSide,
    #[serde(rename = "p")]
    pub price: Decimal,
    pub curr_c: String,
    #[serde(rename = "q")]
    pub quantity: u32,
    pub summ: Decimal,
    pub turnover: String,
    pub order_id: String,
    pub commission: Decimal,
    pub commission_currency: String,
    pub comment: String,
    pub transaction_id: u64,
    pub isin: String,
    pub trade_nb: String,
    pub mkt_name: String,
    pub id: String
}

#[derive(Deserialize)]
pub struct CashFlow {
    pub date: NaiveDate,
    pub account: String,
    pub sum: String,
    pub amount: Decimal,
    pub currency: String,
    pub type_id: String,
    pub comment: String,

}
