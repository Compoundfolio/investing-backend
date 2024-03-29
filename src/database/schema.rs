// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "broker_type"))]
    pub struct BrokerType;

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
        created_at -> Timestamp,
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
    use super::sql_types::CustomMoney;
    use super::sql_types::BrokerType;

    fiscal_transaction (id) {
        id -> Uuid,
        portfolio_id -> Uuid,
        operation_source -> OperationSourceType,
        external_id -> Nullable<Varchar>,
        date_time -> Timestamp,
        symbol_id -> Nullable<Varchar>,
        amount -> CustomMoney,
        operation_type -> Varchar,
        commission -> Nullable<CustomMoney>,
        metadata -> Jsonb,
        broker -> Nullable<BrokerType>,
        report_upload_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    portfolio (id) {
        id -> Uuid,
        app_user_id -> Uuid,
        label -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BrokerType;

    report_upload (id) {
        id -> Uuid,
        portfolio_id -> Uuid,
        label -> Varchar,
        created_at -> Timestamp,
        broker -> BrokerType,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OperationSourceType;
    use super::sql_types::TradeSideType;
    use super::sql_types::CustomMoney;
    use super::sql_types::BrokerType;

    trade_operation (id) {
        id -> Uuid,
        portfolio_id -> Uuid,
        report_upload_id -> Nullable<Uuid>,
        operation_source -> OperationSourceType,
        external_id -> Nullable<Varchar>,
        date_time -> Timestamp,
        side -> TradeSideType,
        instrument_symbol -> Varchar,
        isin -> Nullable<Varchar>,
        price -> CustomMoney,
        quantity -> Int4,
        commission -> Nullable<CustomMoney>,
        order_id -> Nullable<Varchar>,
        summ -> CustomMoney,
        metadata -> Jsonb,
        broker -> Nullable<BrokerType>,
    }
}

diesel::joinable!(app_user_login_method -> app_user (app_user_id));
diesel::joinable!(fiscal_transaction -> portfolio (portfolio_id));
diesel::joinable!(fiscal_transaction -> report_upload (report_upload_id));
diesel::joinable!(portfolio -> app_user (app_user_id));
diesel::joinable!(report_upload -> portfolio (portfolio_id));
diesel::joinable!(trade_operation -> portfolio (portfolio_id));
diesel::joinable!(trade_operation -> report_upload (report_upload_id));

diesel::allow_tables_to_appear_in_same_query!(
    app_user,
    app_user_login_method,
    fiscal_transaction,
    portfolio,
    report_upload,
    trade_operation,
);
