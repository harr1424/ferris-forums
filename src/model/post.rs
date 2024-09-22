use crate::model::comment::Comment;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Post {
    pub id: Uuid,
    pub sub: String,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NewPost {
    pub user_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct PostResponse {
    pub post: Post,
    pub comments: Vec<Comment>,
}
