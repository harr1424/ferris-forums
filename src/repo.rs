use crate::api::comment::Comment;
use crate::api::post::Post;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_post(pool: &PgPool, post: &Post) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO posts (id, sub, author, title, content, timestamp)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        post.id,
        post.sub,
        post.author,
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
        SELECT id, sub, author, title, content, timestamp
        FROM posts
        WHERE id = $1
        "#,
        post_id
    )
    .fetch_one(pool)
    .await?;

    Ok(post)
}

pub async fn create_comment(pool: &PgPool, comment: &Comment) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO comments (id, post_id, author, content, timestamp, parent_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        comment.id,
        comment.post_id,
        comment.author,
        comment.content,
        comment.timestamp,
        comment.parent_id
    )
    .execute(pool)
    .await?;

    Ok(comment.id)
}

pub async fn get_comments_by_post(
    pool: &PgPool,
    post_id: Uuid,
) -> Result<Vec<Comment>, sqlx::Error> {
    let comments = sqlx::query_as!(
        Comment,
        r#"
        SELECT id, post_id, author, content, timestamp, parent_id
        FROM comments
        WHERE post_id = $1
        ORDER BY timestamp ASC
        "#,
        post_id
    )
    .fetch_all(pool)
    .await?;

    Ok(comments)
}
