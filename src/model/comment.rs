use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: i32,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub parent_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct NewComment {
    pub user_id: i32,
    pub content: String,
    pub parent_id: Option<Uuid>,
}
