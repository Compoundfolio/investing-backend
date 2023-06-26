#![forbid(unsafe_code)]

mod datasource;
mod settings;
mod web;

use crate::datasource::diesel::repository::CommonRepository;
use crate::settings::Settings;
use crate::web::model::graphql::QueryRoot;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, routing::get_service, Router};
use axum_server::tls_openssl::OpenSSLConfig;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use std::net::SocketAddr;
use std::sync::Arc;

pub struct ApplicationState {
    pub settings: Settings,
    pub http_client: reqwest::Client,
    pub google_jwt_parser: jsonwebtoken_google::Parser,
    pub repository: CommonRepository,
    pub redis: redis::Client,
}

#[tokio::main]
async fn main() {

    use tracing_subscriber::prelude::*;
    use tracing_subscriber::util::SubscriberInitExt;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let graphsql_schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let settings =
        Settings::from_config_file("config.toml").expect("Expected to read configuration file");
    let state = ApplicationState {
        http_client: reqwest::Client::new(),
        repository: CommonRepository::new(establish_sql_connection(&settings.datasource.sql_url)),
        google_jwt_parser: jsonwebtoken_google::Parser::new(&settings.auth.google.client_id),
        redis: establish_redis_connection(&settings.datasource.redis_url),
        settings,
    };

    let app = Router::new()
        .merge(crate::web::routes::auth::routes())
        .merge(crate::web::routes::graphql::routes())
        .nest_service("/static", get_service(ServeDir::new("./static")))
        .layer(Extension(graphsql_schema))
        .layer(cors)
        .with_state(Arc::new(state));

    let ssl_config = OpenSSLConfig::from_pem_file("./certs/tls.crt", "./certs/tls.key")
        .expect("Could not build open sslf config from pem files");

    let addr = SocketAddr::from(([0, 0, 0, 0], 4430));
    println!("listening on https://{}", addr);
    axum_server::bind_openssl(addr, ssl_config)
        .serve(app.into_make_service())
        .await
        .expect("Should have started the server");
}

pub fn establish_sql_connection(sql_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(sql_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build sql db connection pool")
}

pub fn establish_redis_connection(redus_url: &str) -> redis::Client {
    redis::Client::open(redus_url).expect("Could not create a Redis client")
}
