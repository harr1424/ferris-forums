use crate::model::post::{NewPost, Post, PostResponse};
use crate::model::sub::Sub;
use crate::repo::{comment, post};
use actix_web::http::StatusCode;
use actix_web::HttpResponseBuilder;
use actix_web::{delete, get, patch, post, web::Data, web::Json, web::Path, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[post("/subs/{name}/{description}")]
pub async fn create_sub(
    pool: Data<PgPool>,
    body: Json<Sub>,
) -> Result<HttpResponse, actix_web::Error> {
    todo!();
    // INSERT INTO subs (name, description)
    // VALUES ('example_name', 'This is a description');
}

#[get("/subs")]
pub async fn get_all_subs(pool: Data<PgPool>) -> Result<Json<Vec<Sub>>, actix_web::Error> {
    todo!();
    // SELECT * FROM subs
}

#[patch("/subs")]
pub async fn update_sub(
    pool: Data<PgPool>,
    body: Json<Sub>,
) -> Result<HttpResponse, actix_web::Error> {
    let update_name = &body.name;

    if let Some(update_description) = &body.description {
        // UPDATE subs SET description = update_description WHERE name = update_name
        todo!();
    } else {
        return Ok(HttpResponse::BadRequest().body("Description is missing"));
    }
}
