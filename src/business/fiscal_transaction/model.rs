use chrono::NaiveDateTime;
use diesel::{expression::AsExpression, pg::{Pg, PgValue}, Insertable, Selectable, Queryable};
use diesel::deserialize::{FromSqlRow,FromSql};
use diesel::serialize::{Output,ToSql};
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str,Serialize_enum_str};
use uuid::Uuid;

use crate::{business::model::{BrokerType, Money, OperationSource}, database::schema};

#[derive(Serialize,Deserialize,Insertable,Selectable,Queryable)]
#[diesel(table_name = crate::database::schema::fiscal_transaction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FiscalTransaction {
    pub operation_source: OperationSource,
    pub broker: Option<BrokerType>,
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


// --- orm model

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

