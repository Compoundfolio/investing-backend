use async_graphql::SimpleObject;
use rust_decimal::Decimal;

use diesel::{expression::AsExpression, pg::{Pg, PgValue}, sql_types::Record};
use diesel::deserialize::{FromSqlRow,FromSql};
use diesel::serialize::{Output,ToSql,WriteTuple};
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str,Serialize_enum_str};


use crate::database::schema::{self, sql_types::CustomMoney};




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
    ExanteReport, FreedomfinanceReport, Manual
}


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

