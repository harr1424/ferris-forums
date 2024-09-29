use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Sub {
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}
