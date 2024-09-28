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
    let new_user = NewUser {
        username: body.username.clone(),
        password_hash: User::hash_password(&body.password_hash)
            .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?,
        is_moderator: body.is_moderator,
    };

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

// TODO
// check if username exists
// get user by username
// update user
// delete user
