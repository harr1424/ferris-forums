use crate::model::post::Post;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_post(pool: &PgPool, post: &Post) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO posts (id, sub, user_id, title, content, timestamp)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        post.id,
        post.sub,
        post.user_id,
        post.title,
        post.content,
        post.timestamp,
    )
    .execute(pool)
    .await?;

    Ok(post.id)
}

pub async fn get_post(pool: &PgPool, post_id: Uuid) -> Result<Post, sqlx::Error> {
    let post = sqlx::query_as!(
        Post,
        r#"
        SELECT id, sub, user_id, title, content, timestamp
        FROM posts
        WHERE id = $1
        "#,
        post_id
    )
    .fetch_one(pool)
    .await?;

    Ok(post)
}

pub async fn get_posts_by_sub(pool: &PgPool, sub_name: &str) -> Result<Vec<Post>, sqlx::Error> {
    let posts = sqlx::query_as!(
        Post,
        r#"
        SELECT id, sub, user_id, title, content, timestamp
        FROM posts
        WHERE sub = $1
        "#,
        sub_name
    )
    .fetch_all(pool)
    .await?;

    Ok(posts)
}
