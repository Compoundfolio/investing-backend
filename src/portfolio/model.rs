use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Insertable, expression::AsExpression, deserialize::FromSqlRow, Selectable, Queryable};
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str,Serialize_enum_str};
use uuid::Uuid;

use crate::database::schema;

#[derive(Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::portfolio )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Portfolio {
    pub id: Uuid,
    pub label: String
}

// --- transactions and operations

#[derive(Deserialize_enum_str, Serialize_enum_str)]
#[derive(diesel_derive_enum::DbEnum, Debug)]
#[ExistingTypePath = "crate::database::schema::sql_types::OperationSourceType"]
pub enum AbstractOperationSource {
    ExanteReport, FreedomfinanceReport
}

#[derive(Deserialize_enum_str, Serialize_enum_str)]
#[derive(diesel_derive_enum::DbEnum, Debug)]
#[ExistingTypePath = "crate::database::schema::sql_types::TradeSideType"]
pub enum AbstractTradeSide {
    Buy,
    Sell
}

#[derive(Serialize,Deserialize,Insertable)]
#[diesel(table_name = crate::database::schema::trade_operation )]
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


#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[diesel(sql_type = schema::sql_types::CustomMoney)]
pub struct Money {
    pub value: BigDecimal,
    pub currency: String,
}

impl Money {
    pub fn new(value: BigDecimal, currency: String) -> Self {
        Self { value, currency }
    }
}

// --- orm only

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::trade_operation )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertTradeOperation {
    pub portfolio_id: Uuid,
    #[diesel(embed)]
    pub trade_operation: AbstractTradeOperation
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::portfolio )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertPortfolio<'a> {
    pub app_user_id: Uuid,
    pub label: &'a str
}
