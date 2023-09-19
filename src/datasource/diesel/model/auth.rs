use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::super::enums::LoginMethodType;
use super::super::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::app_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AppUser {
    pub id: Uuid,
    pub email: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::app_user_login_method)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AppUserLoginMethod {
    pub id: Uuid,
    pub app_user_id: Uuid,
    pub login_method_type: LoginMethodType,
    pub subject_id: Option<String>,
    pub password_hash: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::app_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertAppUser<'a> {
    pub email: &'a str,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::app_user_login_method)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertLoginMethod<'a> {
    pub app_user_id: Uuid,
    pub login_method_type: LoginMethodType,
    pub subject_id: Option<&'a str>,
    pub password_hash: Option<&'a str>,
}
