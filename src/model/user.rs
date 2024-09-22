use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)] // Don't serialize password_hash
    pub password_hash: String,
    pub is_moderator: bool,
    pub created_at: DateTime<Utc>,
}
