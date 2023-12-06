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


// --- orm only


#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::portfolio )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertPortfolio<'a> {
    pub app_user_id: Uuid,
    pub label: &'a str
}
