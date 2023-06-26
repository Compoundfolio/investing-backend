use serde::Deserialize;

#[derive(diesel_derive_enum::DbEnum, Debug, Deserialize)]
#[ExistingTypePath = "crate::datasource::diesel::schema::sql_types::LoginMethodTypeType"]
pub enum LoginMethodType {
    GoogleOauth,
    Password,
}
