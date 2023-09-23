#![forbid(unsafe_code)]

mod auth;
mod business;
mod database;
mod portfolio;
mod settings;
mod util;
mod web;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, routing::get_service, Router};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel_migrations::FileBasedMigrations;
use diesel_migrations::HarnessWithOutput;
use diesel_migrations::MigrationHarness;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing::warn;

use std::net::SocketAddr;
use std::sync::Arc;

use crate::settings::Settings;
use crate::web::routes::graphql::QueryRoot;
use crate::database::CommonRepository;

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

    let graphql_schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
    let graphql_schema_sdl = graphql_schema.sdl();
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let settings = Settings::from_config().expect("Expected to read configuration file");

    let port = settings.web.port;

    let state = ApplicationState {
        http_client: reqwest::Client::new(),
        repository: CommonRepository::new(establish_sql_connection(
            &settings.datasource.sql_url,
            settings.datasource.run_migrations,
        )),
        google_jwt_parser: jsonwebtoken_google::Parser::new(&settings.auth.google.client_id),
        redis: establish_redis_connection(&settings.datasource.redis_url),
        settings,
    };

    let app = Router::new()
        .merge(crate::auth::routes::routes())
        .merge(crate::web::routes::graphql::routes())
        .route(
            "/graphql/sdl",
            axum::routing::get(|| async { graphql_schema_sdl }),
        )
        .nest_service("/static", get_service(ServeDir::new("./static")))
        .layer(Extension(graphql_schema))
        .layer(cors)
        .layer(axum::middleware::from_fn(middleware_add_version_header))
        .with_state(Arc::new(state));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Starting server on on http://{}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .expect("Should have started the http server");
}

pub fn establish_sql_connection(
    sql_url: &str,
    run_migrations: bool,
) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(sql_url);

    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build sql db connection pool");

    if run_migrations {
        let mut conn = pool
            .get()
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

async fn middleware_add_version_header<B>(
    request: axum::http::Request<B>,
    next: axum::middleware::Next<B>,
) -> axum::response::Response {
    let mut response = next.run(request).await;
    let mut description = String::new();
    if let Some(name) = option_env!("CARGO_PKG_NAME") {
        description.push_str(name);
        description.push('/');
    } else {
        description.push_str("axum-based ");
    }
    if let Some(version) = option_env!("CARGO_PKG_VERSION") {
        description.push('v');
        description.push_str(version);
    }
    match description.parse() {
        Ok(header) => {
            response
                .headers_mut()
                .insert(axum::http::header::SERVER, header);
        }
        Err(e) => {
            warn!("Could not add version header because its value is not usable: {e}");
        }
    };
    response
}
