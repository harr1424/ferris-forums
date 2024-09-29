use crate::model::user::{DbAddUser, User};
use sqlx::PgPool; // For password hashing

pub async fn create_user(pool: &PgPool, user: &DbAddUser) -> Result<i32, sqlx::Error> {
    let user = sqlx::query!(
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

    Ok(user.id)
}

pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, is_moderator, created_at
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
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

pub async fn get_users_by_sub(pool: &PgPool, sub_name: &str) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT users.*
        FROM users
        INNER JOIN subscriptions ON users.id = subscriptions.user_id
        WHERE subscriptions.sub_name = $1
        "#,
        sub_name
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn username_exists(pool: &PgPool, username: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, is_moderator, created_at
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn grant_mod_status(pool: &PgPool, user_id: i32) -> Result<i32, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET is_moderator = true
        WHERE id = $1
        "#,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(user_id)
}

pub async fn remove_mod_status(pool: &PgPool, user_id: i32) -> Result<i32, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET is_moderator = false
        WHERE id = $1
        "#,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(user_id)
}

pub async fn update_user_password(
    pool: &PgPool,
    user_id: i32,
    new_hash: &str,
) -> Result<i32, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET password_hash = $1
        WHERE id = $2
        "#,
        new_hash,
        user_id,
    )
    .execute(pool)
    .await?;

    Ok(user_id)
}

pub async fn delete_user(pool: &PgPool, user_id: i32) -> Result<i32, sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(user_id)
}
