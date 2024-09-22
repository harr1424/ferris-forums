use crate::model::comment::Comment;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_comment(pool: &PgPool, comment: &Comment) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO comments (id, post_id, user_id, content, timestamp, parent_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        comment.id,
        comment.post_id,
        comment.user_id,
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
        SELECT id, post_id, user_id, content, timestamp, parent_id
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
