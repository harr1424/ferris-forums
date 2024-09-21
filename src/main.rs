mod api;
mod repo;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::comment::{create_comment, get_comments};
use api::post::{create_post, get_post};
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
            .service(get_post)
            .service(create_post)
            .service(get_comments)
            .service(create_comment)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
