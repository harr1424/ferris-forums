use crate::model::user::User;
use sqlx::PgPool; // For password hashing

pub async fn create_user(pool: &PgPool, user: &User) -> Result<i32, sqlx::Error> {
    let record = sqlx::query!(
        r#"
        INSERT INTO users (username, password_hash, is_moderator, created_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        user.username,
        user.password_hash,
        user.is_moderator,
        user.created_at,
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id)
}

pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, is_moderator, created_at
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}
