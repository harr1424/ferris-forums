use crate::model::post::{NewPost, Post, PostResponse};
use crate::repo::{comment, post};
use actix_web::{delete, get, patch, post, web::Data, web::Json, web::Path, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[post("/posts/{sub}")]
pub async fn create_post(
    pool: Data<PgPool>,
    sub: Path<String>,
    body: Json<NewPost>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_post = Post {
        id: Uuid::new_v4(),
        sub: sub.into_inner(),
        user_id: body.user_id,
        title: body.title.clone(),
        content: body.content.clone(),
        timestamp: Utc::now(),
    };

    let post_id = post::create_post(&pool, &new_post)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body(post_id.to_string()))
}

#[get("/posts/{id}")]
pub async fn get_post(
    pool: Data<PgPool>,
    path: Path<Uuid>,
) -> Result<Json<PostResponse>, actix_web::Error> {
    let post_id = path.into_inner();
    let post = post::get_post(&pool, post_id)
        .await
        .map_err(|e| actix_web::error::ErrorNotFound(e))?;
    let comments = comment::get_comments_by_post(&pool, post_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(Json(PostResponse { post, comments }))
}

#[get("/posts/{sub}")]
pub async fn get_posts_by_sub(
    pool: Data<PgPool>,
    sub: Path<String>,
) -> Result<Json<Vec<Post>>, actix_web::Error> {
    todo!();
    // SELECT * FROM posts WHERE sub = sub
}

#[patch("/posts/{id}")]
pub async fn update_post(
    pool: Data<PgPool>,
    path: Path<Uuid>,
    update_content: String,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id = path.into_inner();

    todo!();
    // UPDATE posts SET content = update_content WHERE id = post_id

    Ok(HttpResponse::Ok().body(post_id.to_string()))
}

#[delete("/posts/{id}")]
pub async fn delete_post(
    pool: Data<PgPool>,
    path: Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id = path.into_inner();

    todo!();
    // DELETE FROM posts where id = post_id

    Ok(HttpResponse::Ok().body(post_id.to_string()))
}
