use crate::model::sub::Sub;
use sqlx::PgPool;

pub async fn create_sub(pool: &PgPool, sub: &Sub) -> Result<String, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subs (name, description, created_at)
        VALUES ($1, $2, $3)
        "#,
        sub.name,
        sub.description,
        sub.created_at,
    )
    .execute(pool)
    .await?;

    Ok(sub.name.clone())
}

pub async fn get_all_subs(pool: &PgPool) -> Result<Vec<Sub>, sqlx::Error> {
    let subs = sqlx::query_as!(
        Sub,
        r#"
        SELECT * FROM subs
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(subs)
}

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

pub async fn get_subs_by_user_id(pool: &PgPool, user_id: i32) -> Result<Vec<Sub>, sqlx::Error> {
    let subs = sqlx::query_as!(
        Sub,
        r#"
        SELECT subs.*
        FROM subs
        INNER JOIN subscriptions ON subs.name = subscriptions.sub_name
        WHERE subscriptions.user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(subs)
}

pub async fn update_sub(pool: &PgPool, sub: &Sub) -> Result<(String, String), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE subs
        SET description = $1
        WHERE name = $2
        "#,
        sub.description,
        sub.name,
    )
    .execute(pool)
    .await?;

    Ok((sub.name.clone(), sub.description.clone()))
}

pub async fn delete_sub(pool: &PgPool, name: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM subs
        WHERE name = $1
        "#,
        name
    )
    .execute(pool)
    .await?;

    Ok(())
}
