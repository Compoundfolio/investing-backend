#![forbid(unsafe_code)]

mod datasource;
mod settings;
mod web;

use crate::datasource::diesel::repository::CommonRepository;
use crate::settings::Settings;
use crate::web::routes::graphql::QueryRoot;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, routing::get_service, Router};
use axum_server::tls_openssl::OpenSSLConfig;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel_migrations::FileBasedMigrations;
use diesel_migrations::MigrationHarness;
use diesel_migrations::HarnessWithOutput;
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

    tracing_subscriber::fmt()
          .with_max_level(tracing::Level::INFO)
          .init();

    let graphsql_schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers(Any);

    let settings =
        Settings::from_config().expect("Expected to read configuration file");

    let use_ssl = settings.web.use_ssl;
    let port = settings.web.port;

    let state = ApplicationState {
        http_client: reqwest::Client::new(),
        repository: CommonRepository::new(establish_sql_connection(
                &settings.datasource.sql_url, 
                settings.datasource.run_migrations
                )),
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


    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);

    if use_ssl {
        let ssl_config = OpenSSLConfig::from_pem_file("./certs/tls.crt", "./certs/tls.key")
            .expect("Could not build open sslf config from pem files");
        axum_server::bind_openssl(addr, ssl_config)
            .serve(app.into_make_service())
            .await
            .expect("Should have started the https server");
    } else { 
        axum_server::bind(addr)
            .serve(app.into_make_service())
            .await
            .expect("Should have started the http server");
    };
}

pub fn establish_sql_connection(sql_url: &str, run_migrations: bool) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(sql_url);

    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build sql db connection pool");

    if run_migrations {
        let mut conn = pool.get()
            .expect("Could not get connection from pool to run migrations");

        let migrations = FileBasedMigrations::find_migrations_directory()
            .expect("Could not find migrationsto run");
        HarnessWithOutput::write_to_stdout(&mut conn)
            .run_pending_migrations(migrations)
            .expect("There was an error running migrations");
    }

    pool
}

pub fn establish_redis_connection(redus_url: &str) -> redis::Client {
    redis::Client::open(redus_url).expect("Could not create a Redis client")
}
