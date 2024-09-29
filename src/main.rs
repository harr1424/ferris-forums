mod api;
mod model;
mod repo;
mod routing;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

use sqlx::postgres::PgPoolOptions;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env or environment variables");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Could not connect to the database");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(Data::new(pool.clone()))
            .configure(routing::configure_post_routes)
            .configure(routing::configure_comment_routes)
            .configure(routing::configure_user_routes)
            .configure(routing::configure_sub_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
