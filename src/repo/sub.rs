use crate::model::sub::Sub;
use sqlx::PgPool;

pub async fn get_sub_by_name(pool: &PgPool, name: &str) -> Result<Sub, sqlx::Error> {
    let sub = sqlx::query_as!(
        Sub,
        r#"
        SELECT name, description, created_at
        FROM subs
        WHERE name = $1
        "#,
        name
    )
    .fetch_one(pool)
    .await?;

    Ok(sub)
}
