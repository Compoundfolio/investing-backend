// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "login_method_type_type"))]
    pub struct LoginMethodTypeType;
}

diesel::table! {
    app_user (id) {
        id -> Uuid,
        email -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::LoginMethodTypeType;

    app_user_login_method (id) {
        id -> Uuid,
        app_user_id -> Uuid,
        login_method_type -> LoginMethodTypeType,
        subject_id -> Nullable<Varchar>,
        password_hash -> Nullable<Varchar>,
    }
}

diesel::joinable!(app_user_login_method -> app_user (app_user_id));

diesel::allow_tables_to_appear_in_same_query!(app_user, app_user_login_method,);
