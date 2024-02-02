use std::io::Write;

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Insertable, expression::AsExpression, pg::{Pg, PgValue}, sql_types::{VarChar, Record}, backend::Backend};
use diesel::deserialize::{FromSqlRow,FromSql};
use diesel::serialize::{Output,ToSql,WriteTuple};
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str,Serialize_enum_str};
use uuid::Uuid;

use crate::database::schema::{self, sql_types::CustomMoney};

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
    pub broker: BrokerType,
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

#[derive(Serialize,Deserialize,Insertable,Debug)]
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
    Dividend,
    Trade,
    Commission,
    FundingWithdrawal,
    RevertedDividend,
    #[serde(other)]
    Unrecognized(String),
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[diesel(sql_type = schema::sql_types::CustomMoney)]
pub struct Money {
    pub amount: BigDecimal,
    pub currency: String,
}

impl Money {
    pub fn new(amount: BigDecimal, currency: String) -> Self {
        Self { amount, currency }
    }
}


pub struct ReportProcessingResult {
    pub id: Uuid,
    pub transactions: usize,
    pub trade_operations: usize
}

#[derive(thiserror::Error, Debug)]
pub enum ReportProcessingError {
    #[error(transparent)]
    ExanteReportParsingError { #[from] source: super::exante::model::ExanteReportParsingError },
    #[error(transparent)]
    FreedomfinanceReportParsingError { #[from] source: super::freedomfinance::model::FreedomfinanceReportParsingError },
}


// --- orm model

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::report_upload )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertReportUpload {
    pub portfolio_id: Uuid,
    pub label: String,
    pub broker: BrokerType
}


#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::trade_operation )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertTradeOperation {
    pub portfolio_id: Uuid,
    pub report_upload_id: Uuid,
    #[diesel(embed)]
    pub trade_operation: AbstractTradeOperation
}

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = schema::transaction )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertTransaction {
    pub portfolio_id: Uuid,
    pub report_upload_id: Uuid,
    #[diesel(embed)]
    pub transaction: AbstractTransaction
}



// --- orm implementations

// TODO: When it works, make a post solution here:
// https://github.com/diesel-rs/diesel/issues/1732
// Thanks to this blogpost
// https://inve.rs/postgres-diesel-composite/

impl ToSql<CustomMoney, Pg> for Money {
    fn to_sql(&self, out: &mut Output<Pg>) -> diesel::serialize::Result {
        WriteTuple::<(diesel::sql_types::Numeric, diesel::sql_types::Text)>::write_tuple(&(self.amount.clone(), self.currency.clone()), out)
    }
}

impl FromSql<CustomMoney, Pg> for Money {
    fn from_sql(input: PgValue) -> diesel::deserialize::Result<Self> {
        let (value, currency) = FromSql::<Record<(diesel::sql_types::Numeric, diesel::sql_types::Text)>, Pg>::from_sql(input)?;
        Ok(Money { amount: value, currency })
    }
}

impl ToSql<diesel::sql_types::Text, Pg> for AbstractTransactionType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let string_repr = self.to_string();
        ToSql::<diesel::sql_types::Text, Pg>::to_sql(&string_repr, &mut out.reborrow())
    }
}

impl FromSql<diesel::sql_types::Text, Pg>  for AbstractTransactionType  {
    fn from_sql(input: PgValue) -> diesel::deserialize::Result<Self> {
        match FromSql::<diesel::sql_types::Text, Pg>::from_sql(input).map(|v: String| Self::try_from(v)) {
            Ok(o) => Ok(o?),
            Err(e) => Err(e),
        }
    }
}
