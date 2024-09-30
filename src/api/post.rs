use crate::model::post::{NewPost, Post, PostResponse};
use crate::repo::{comment as comment_repo, post as post_repo};
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

    let post_id = post_repo::create_post(&pool, &new_post)
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
    let post = post_repo::get_post(&pool, post_id)
        .await
        .map_err(|e| actix_web::error::ErrorNotFound(e))?;
    let comments = comment_repo::get_comments_by_post(&pool, post_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(Json(PostResponse { post, comments }))
}

#[get("/posts/for_sub/{sub}")]
pub async fn get_posts_by_sub(
    pool: Data<PgPool>,
    sub: Path<String>,
) -> Result<Json<Vec<Post>>, actix_web::Error> {
    let sub_name = sub.into_inner();

    let posts = post_repo::get_posts_by_sub(&pool, &sub_name)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(Json(posts))
}

#[patch("/posts/{id}")]
pub async fn update_post(
    pool: Data<PgPool>,
    path: Path<Uuid>,
    update_content: String,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id = path.into_inner();

    let post_id = post_repo::update_post(&pool, post_id, update_content.clone())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body(format!("{} -> {}", post_id.to_string(), update_content)))
}

#[delete("/posts/{id}")]
pub async fn delete_post(
    pool: Data<PgPool>,
    path: Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let post_id = path.into_inner();

    let post_id = post_repo::delete_post(&pool, post_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body(format!("{:?} was deleted", post_id)))
}
