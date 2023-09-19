// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "custom_money"))]
    pub struct CustomMoney;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "login_method_type_type"))]
    pub struct LoginMethodTypeType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "operation_source_type"))]
    pub struct OperationSourceType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "trade_side_type"))]
    pub struct TradeSideType;
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

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OperationSourceType;
    use super::sql_types::TradeSideType;
    use super::sql_types::CustomMoney;

    trade_operation (id) {
        id -> Uuid,
        app_user_id -> Uuid,
        operation_source -> OperationSourceType,
        external_id -> Varchar,
        date_time -> Timestamp,
        side -> TradeSideType,
        instrument_symbol -> Varchar,
        isin -> Varchar,
        price -> CustomMoney,
        quantity -> Nullable<Int4>,
        commission -> Nullable<CustomMoney>,
        order_id -> Varchar,
        summ -> CustomMoney,
        metadata -> Jsonb,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OperationSourceType;
    use super::sql_types::CustomMoney;

    transaction (id) {
        id -> Uuid,
        app_user_id -> Uuid,
        operation_source -> OperationSourceType,
        external_id -> Varchar,
        date_time -> Timestamp,
        symbol_id -> Nullable<Varchar>,
        amount -> CustomMoney,
        operation_type -> Varchar,
        commission -> CustomMoney,
        metadata -> Jsonb,
    }
}

diesel::joinable!(app_user_login_method -> app_user (app_user_id));
diesel::joinable!(trade_operation -> app_user (app_user_id));
diesel::joinable!(transaction -> app_user (app_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    app_user,
    app_user_login_method,
    trade_operation,
    transaction,
);
