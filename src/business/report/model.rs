use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Insertable, expression::AsExpression, deserialize::FromSqlRow, Selectable, Queryable};
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str,Serialize_enum_str};
use uuid::Uuid;

use crate::database::schema;

// --- transactions and trade operations

#[derive(Deserialize_enum_str, Serialize_enum_str)]
#[derive(diesel_derive_enum::DbEnum, Debug, async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
#[ExistingTypePath = "crate::database::schema::sql_types::BrokerType"]
pub enum BrokerType {
    Exante, Freedomfinance
}

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


#[derive(Serialize)]
pub struct AbstractReport {
    pub trade_operations: Vec<AbstractTradeOperation>,
    pub transactions: Vec<AbstractTransaction>
}

#[derive(Serialize,Deserialize,Insertable)]
#[diesel(table_name = crate::database::schema::trade_operation)]
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

#[derive(Serialize,Deserialize,Insertable)]
#[diesel(table_name = crate::database::schema::transaction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AbstractTransaction {
    pub operation_source: AbstractOperationSource,
    pub external_id: Option<String>,
    pub date_time: NaiveDateTime,
    pub symbol_id: Option<String>,
    pub amount: Money,
    pub operation_type: AbstractTransactionType,
    pub commission: Option<Money>,
    pub metadata: serde_json::Value,
}

#[derive(Deserialize_enum_str, Serialize_enum_str)]
#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Varchar)]
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

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::trade_operation )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertTradeOperation {
    pub portfolio_id: Uuid,
    #[diesel(embed)]
    pub trade_operation: AbstractTradeOperation
}

#[derive(thiserror::Error, Debug)]
pub enum AbstractReportParseError {
    #[error(transparent)]
    ExanteReportParsingError { #[from] source: super::exante::model::ExanteReportParsingError },
    #[error(transparent)]
    FreedomfinanceReportParsingError { #[from] source: super::freedomfinance::model::FreedomfinanceReportParsingError },
}

// --- orm implementations

// impl ToSql<Varchar, PgConnection> for MyEnum {
//     fn to_sql<W: Write>(&self, out: &mut Output<W, PgConnection>) -> serialize::Result {
//         match *self {
//             MyEnum::KnownVariant => out.write_all(b"KnownVariant")?,
//             MyEnum::Unrecognized(ref s) => out.write_all(s.as_bytes())?,
//         }
//         Ok(IsNull::No)
//     }
// }
