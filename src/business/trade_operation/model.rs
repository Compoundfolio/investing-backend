

use chrono::NaiveDateTime;
use diesel::{Insertable, Selectable, Queryable};


use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str,Serialize_enum_str};
use uuid::Uuid;

use crate::{business::model::{BrokerType, Money, OperationSource}, database::schema};

#[derive(Deserialize_enum_str, Serialize_enum_str)]
#[derive(diesel_derive_enum::DbEnum, Debug, async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
#[ExistingTypePath = "crate::database::schema::sql_types::TradeSideType"]
pub enum TradeOperationSide {
    Buy,
    Sell
}

#[derive(Serialize,Deserialize,Insertable,Selectable,Queryable)]
#[diesel(table_name = crate::database::schema::trade_operation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TradeOperation {
    pub operation_source: OperationSource,
    pub broker: Option<BrokerType>,
    pub external_id: Option<String>,
    pub date_time: NaiveDateTime,
    pub side: TradeOperationSide,
    pub instrument_symbol: String,
    pub isin: Option<String>,
    pub price: Money,
    pub quantity: i32,
    pub commission: Option<Money>,
    pub order_id: Option<String>,
    pub summ: Money, // always positive traded volume without comission
    pub metadata: serde_json::Value,
}

// --- orm model

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::trade_operation )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertTradeOperation {
    pub portfolio_id: Uuid,
    pub report_upload_id: Option<Uuid>,
    #[diesel(embed)]
    pub trade_operation: TradeOperation
}

#[derive(Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::trade_operation )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SelectTradeOperation {
    pub id: Uuid,
    pub portfolio_id: Uuid,
//    pub report_upload_id: Option<Uuid>,
    #[diesel(embed)]
    pub i: TradeOperation
}
