use crate::model::user::{NewUser, User};
use crate::repo::{comment, post};
use actix_web::http::StatusCode;
use actix_web::HttpResponseBuilder;
use actix_web::{delete, get, patch, post, web::Data, web::Json, web::Path, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[post("/users")]
pub async fn create_user(
    pool: Data<PgPool>,
    body: Json<NewUser>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let username = body.username.clone();
    let password_hash = User::hash_password(&body.password)
        .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;
    let is_moderator = body.is_moderator;

    todo!();
    // repo method

    Ok(HttpResponse::Ok().body(body.username))
}

#[get("/users/{sub_name}")]
pub async fn get_users_by_sub(
    pool: Data<PgPool>,
    path: Path<String>,
) -> Result<Json<Vec<User>>, actix_web::Error> {
    let sub = path.into_inner();

    todo!();
    // SELECT users.*
    // FROM users
    // INNER JOIN subscriptions ON users.id = subscriptions.user_id
    // WHERE subscriptions.sub_name = 'example_sub'; -- Replace 'example_sub' with the sub_name
}

#[get("/users/{user_id}")]
pub async fn get_user_by_id(
    pool: Data<PgPool>,
    path: Path<i32>,
) -> Result<Json<User>, actix_web::Error> {
    let user_id = path.into_inner();

    todo!();
    // SELECT * FROM users WHERE id = user_id
}

#[get("/users/exists/{username}")]
pub async fn username_exists(
    pool: Data<PgPool>,
    path: Path<String>,
) -> Result<Json<Vec<User>>, actix_web::Error> {
    let username = path.into_inner();

    todo!();
    // SELECT COUNT( SELECT id FROM users WHERE username = username)
}

#[patch("/users/mods/add/{user_id}")]
pub async fn grant_mod_status(
    pool: Data<PgPool>,
    path: Path<i32>,
) -> Result<Json<User>, actix_web::Error> {
    let user_id = path.into_inner();

    todo!();
    // UPDATE users SET is_moderator = TRUE WHERE id = user_id
}

#[patch("/users/mods/remove/{user_id}")]
pub async fn remove_mod_status(
    pool: Data<PgPool>,
    path: Path<i32>,
) -> Result<Json<User>, actix_web::Error> {
    let user_id = path.into_inner();

    todo!();
    // UPDATE users SET is_moderator = FALSE WHERE id = user_id
}

// TODO
// update user password
#[patch("/users/creds/{user_id}")]
pub async fn update_user_password(
    pool: Data<PgPool>,
    path: Path<i32>,
    body: String,
) -> Result<Json<User>, actix_web::Error> {
    let user_id = path.into_inner();
    let new_password_hash = User::hash_password(&body)
        .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;
    todo!();
    // UPDATE users SET password_hash = new_password_hash WHERE id = user_id
}

#[delete("/users/{user_id}")]
pub async fn delete_user(
    pool: Data<PgPool>,
    path: Path<i32>,
) -> Result<Json<User>, actix_web::Error> {
    let user_id = path.into_inner();

    todo!();
    // DELETE FROM users WHERE id = user_id
}
