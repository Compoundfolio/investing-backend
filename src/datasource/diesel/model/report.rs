use bigdecimal::BigDecimal;
use diesel::{prelude::*, expression::AsExpression, deserialize::FromSqlRow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::business::report::model::AbstractTradeOperation;
use super::super::schema;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[diesel(sql_type = super::super::schema::sql_types::CustomMoney)]
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
    pub app_user_id: Uuid,
    #[diesel(embed)]
    pub trade_operation: AbstractTradeOperation
}
