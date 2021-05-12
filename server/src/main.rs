#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use actix_web::{web, App, HttpServer};

use hmac::{Hmac, NewMac};

mod auth;
pub use auth::middleware::Authorized;

mod announcement;
mod chat;
mod event;
mod image;
mod user;

mod config;
pub use config::Config;

pub mod actions;
pub mod custom_schema_types;
pub mod model;
pub mod schema;

#[macro_use]
pub mod error;
pub mod logging;
pub mod util;

use clap::{crate_authors, crate_version, load_yaml};
use slog::info;
use sloggers::{types::Severity, Build};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type RedisPool = bb8_redis::bb8::Pool<bb8_redis::RedisConnectionManager>;
pub type RedisConnection = bb8_redis::redis::aio::Connection;

embed_migrations!("migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::{path::PathBuf, time::Duration};
    // Load the local .env file
    dotenv::dotenv().ok();

    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from(yaml)
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();

    let logger = if let Some(log_path) = matches.value_of("log") {
        use slog::Drain;
        let file_drain = sloggers::file::FileLoggerBuilder::new(log_path)
            .level(Severity::Trace)
            .rotate_size(5_000_000)
            .rotate_keep(100)
            .rotate_compress(true)
            .build()
            .unwrap();

        let terminal_drain = sloggers::terminal::TerminalLoggerBuilder::new()
            .level(Severity::Info)
            .build()
            .unwrap();

        let combined_drain = slog::Duplicate::new(file_drain, terminal_drain);

        slog::Logger::root(combined_drain.fuse(), slog::o!())
    } else {
        sloggers::terminal::TerminalLoggerBuilder::new()
            .level(Severity::Trace)
            .build()
            .unwrap()
    };

    let connspec =
        std::env::var("DATABASE_URL").expect("The DATABASE_URL env var must be set (or in .env)");
    info!(logger, "Connecting to pool (postgres)");
    let pg_manager = ConnectionManager::<PgConnection>::new(connspec);
    let pg_pool = web::Data::new(
        r2d2::Pool::builder()
            .build(pg_manager)
            .expect("Failed to create pool."),
    );
    info!(logger, "Connecting to pool (redis)");

    let redis_manager = bb8_redis::RedisConnectionManager::new("redis://localhost")
        .expect("Couldn't create redis connection manager");
    let redis_pool = web::Data::new(
        bb8_redis::bb8::Pool::builder()
            .build(redis_manager)
            .await
            .expect("Couldn't create redis connection pool"),
    );

    info!(logger, "connected, running migrations");

    embedded_migrations::run_with_output(
        &pg_pool.get().expect("Couldn't connect to the database"),
        &mut std::io::stdout(),
    )
    .expect("Couldn't setup db");

    info!(logger, "Loading private config");
    let private_config = config::PrivateConfig::load_from_file("private_config.json")
        .expect("A private_config.json must be provided");

    let config = web::Data::new(Config {
        // Check for secrets file in env or create one randomly seeded
        // if it doesn't exist.
        jwt_secret: Hmac::new_varkey(private_config.jwt_secret.as_bytes())
            .expect("Couldn't create jwt secret key"),
        jwt_lifetime: Duration::from_secs(60 * 60 * 24 * 7), // 1 week
        sso_timeout: Duration::from_secs(60 * 15),           // 15 mins
        sso_extension: Duration::from_secs(60 * 5),          // 5 mins
        sso_redirect_url: private_config.sso_redirect_url,
        oauth_client_id: private_config.oauth_client_id,
        oauth_client_secret: private_config.oauth_client_secret,
        image_dir: PathBuf::from("/etc/tibi/images"),
    });

    // Get the port specified as a command line option or default to 8080 if it doesn't exist
    let port = matches
        .value_of("port")
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    info!(logger, "Starting server on http://localhost:{}", port);

    let logger_data = web::Data::new(logger.clone());

    // The HTTPServer will run the provided closure for as many threads as there are on the
    // host machine. Any data created within the closure is local to the thread and not to the
    // application on whole.
    HttpServer::new(move || {
        App::new()
            .wrap(logging::Logging::new(logger.clone()))
            .app_data(config.clone())
            .app_data(pg_pool.clone())
            .app_data(redis_pool.clone())
            .app_data(logger_data.clone())
            .configure(auth::add_endpoints)
            .configure(user::add_endpoints)
            .configure(image::add_endpoints)
            .configure(announcement::add_endpoints)
            .configure(event::add_endpoints)
            .configure(chat::add_endpoints)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
