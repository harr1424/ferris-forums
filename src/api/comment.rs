use crate::repo;
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub author: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub parent_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct NewComment {
    author: String,
    content: String,
    parent_id: Option<Uuid>,
}

#[get("/posts/{post_id}/comments")]
pub async fn get_comments(pool: Data<PgPool>, path: Path<Uuid>) -> Result<Json<Vec<Comment>>> {
    let post_id = path.into_inner();
    let comments = repo::get_comments_by_post(&pool, post_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(Json(comments))
}

#[post("/posts/{post_id}/comments")]
pub async fn create_comment(
    pool: Data<PgPool>,
    path: Path<Uuid>,
    body: Json<NewComment>,
) -> Result<HttpResponse> {
    let post_id = path.into_inner();
    let comment = Comment {
        id: Uuid::new_v4(),
        post_id,
        author: body.author.clone(),
        content: body.content.clone(),
        timestamp: Utc::now(),
        parent_id: body.parent_id,
    };

    let comment_id = repo::create_comment(&pool, &comment)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body(comment_id.to_string()))
}
