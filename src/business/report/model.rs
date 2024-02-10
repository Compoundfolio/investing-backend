use async_graphql::SimpleObject;
use rust_decimal::Decimal;
use chrono::NaiveDateTime;
use diesel::{expression::AsExpression, pg::{Pg, PgValue}, sql_types::Record, Insertable, Selectable, Queryable};
use diesel::deserialize::{FromSqlRow,FromSql};
use diesel::serialize::{Output,ToSql,WriteTuple};
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str,Serialize_enum_str};
use uuid::Uuid;

use crate::database::schema::{self, sql_types::CustomMoney};

// --- money

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize, SimpleObject)]
#[diesel(sql_type = schema::sql_types::CustomMoney)]
pub struct Money {
    pub amount: Decimal,
    pub currency: String,
}

impl Money {
    pub fn new(amount: Decimal, currency: String) -> Self {
        Self { amount, currency }
    }
}

impl std::ops::Mul<i32> for Money {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self::new(self.amount * Decimal::from(rhs), self.currency)
    }
}


// --- fiscal transactions and trade operations

#[derive(Deserialize_enum_str, Serialize_enum_str)]
#[derive(diesel_derive_enum::DbEnum, Debug, async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
#[ExistingTypePath = "crate::database::schema::sql_types::BrokerType"]
pub enum BrokerType {
    Exante, Freedomfinance
}

#[derive(Deserialize_enum_str, Serialize_enum_str)]
#[derive(diesel_derive_enum::DbEnum, Debug)]
#[ExistingTypePath = "crate::database::schema::sql_types::OperationSourceType"]
pub enum OperationSource {
    ExanteReport, FreedomfinanceReport
}

#[derive(Deserialize_enum_str, Serialize_enum_str)]
#[derive(diesel_derive_enum::DbEnum, Debug)]
#[ExistingTypePath = "crate::database::schema::sql_types::TradeSideType"]
pub enum TradeOperationSide {
    Buy,
    Sell
}


#[derive(Serialize)]
pub struct AbstractReport {
    pub broker: BrokerType,
    pub trade_operations: Vec<TradeOperation>,
    pub fiscal_transactions: Vec<FiscalTransaction>
}

#[derive(Serialize,Deserialize,Insertable,Selectable,Queryable)]
#[diesel(table_name = crate::database::schema::trade_operation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TradeOperation {
    pub operation_source: OperationSource,
    pub external_id: Option<String>,
    pub date_time: NaiveDateTime,
    pub side: TradeOperationSide,
    pub instrument_symbol: String,
    pub isin: String,
    pub price: Money,
    pub quantity: i32,
    pub commission: Option<Money>,
    pub order_id: String,
    pub summ: Money, // always positive traded volume without comission
    pub metadata: serde_json::Value,
}

#[derive(Serialize,Deserialize,Insertable,Selectable,Queryable)]
#[diesel(table_name = crate::database::schema::fiscal_transaction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FiscalTransaction {
    pub operation_source: OperationSource,
    pub external_id: Option<String>,
    pub date_time: NaiveDateTime,
    pub symbol_id: Option<String>,
    pub amount: Money,
    pub operation_type: FiscalTransactionType,
    pub commission: Option<Money>,
    pub metadata: serde_json::Value,
}

#[derive(Deserialize_enum_str, Serialize_enum_str)]
#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Varchar)]
pub enum FiscalTransactionType {
    Tax,
    Dividend,
    Commission,
    FundingWithdrawal,
    RevertedDividend,
    #[serde(other)]
    Unrecognized(String),
}



pub struct ReportProcessingResult {
    pub id: Uuid,
    pub fiscal_transactions: usize,
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
    pub trade_operation: TradeOperation
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::fiscal_transaction )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertFiscalTransaction {
    pub portfolio_id: Uuid,
    pub report_upload_id: Uuid,
    #[diesel(embed)]
    pub fiscal_transaction: FiscalTransaction
}

#[derive(Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::trade_operation )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SelectTradeOperation {
    pub id: Uuid,
//    pub portfolio_id: Uuid,
//    pub report_upload_id: Option<Uuid>,
    #[diesel(embed)]
    pub i: TradeOperation
}

#[derive(Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::fiscal_transaction )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SelectFiscalTransaction {
    pub id: Uuid,
//    pub portfolio_id: Uuid,
//    pub report_upload_id: Option<Uuid>,
    #[diesel(embed)]
    pub i: FiscalTransaction
}



// --- orm implementations

// TODO: When it works, make a post solution here:
// https://github.com/diesel-rs/diesel/issues/1732
// Thanks to this blogpost
// https://inve.rs/postgres-diesel-composite/

#[allow(clippy::clone_on_copy)]
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

impl ToSql<diesel::sql_types::Text, Pg> for FiscalTransactionType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let string_repr = self.to_string();
        ToSql::<diesel::sql_types::Text, Pg>::to_sql(&string_repr, &mut out.reborrow())
    }
}

#[allow(clippy::redundant_closure)]
impl FromSql<diesel::sql_types::Text, Pg>  for FiscalTransactionType  {
    fn from_sql(input: PgValue) -> diesel::deserialize::Result<Self> {
        match FromSql::<diesel::sql_types::Text, Pg>::from_sql(input).map(|v: String| Self::try_from(v)) {
            Ok(o) => Ok(o?),
            Err(e) => Err(e),
        }
    }
}
