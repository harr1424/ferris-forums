use crate::api::comment::Comment;
use crate::repo;
use actix_web::{get, post, web::Data, web::Json, web::Path, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Post {
    pub id: Uuid,
    pub sub: String,
    pub author: String,
    pub title: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NewPost {
    pub author: String,
    pub title: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct PostResponse {
    pub post: Post,
    pub comments: Vec<Comment>,
}

#[get("/posts/{id}")]
pub async fn get_post(
    pool: Data<PgPool>,
    path: Path<Uuid>,
) -> Result<Json<PostResponse>, actix_web::Error> {
    let post_id = path.into_inner();
    let post = repo::get_post(&pool, post_id)
        .await
        .map_err(|e| actix_web::error::ErrorNotFound(e))?;
    let comments = repo::get_comments_by_post(&pool, post_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(Json(PostResponse { post, comments }))
}

#[post("/post/{sub}")]
pub async fn create_post(
    pool: Data<PgPool>,
    sub: Path<String>,
    body: Json<NewPost>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_post = Post {
        id: Uuid::new_v4(),
        sub: sub.into_inner(),
        author: body.author.clone(),
        title: body.title.clone(),
        content: body.content.clone(),
        timestamp: Utc::now(),
    };

    // Save the new post to the database
    let post_id = repo::create_post(&pool, &new_post)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // Return the post ID as the response
    Ok(HttpResponse::Ok().body(post_id.to_string()))
}
