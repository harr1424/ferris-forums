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

pub async fn update_comment(
    pool: &PgPool,
    comment_id: Uuid,
    new_comment: String,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE comments
        SET content = $1
        WHERE id = $2
        "#,
        new_comment,
        comment_id,
    )
    .execute(pool)
    .await?;

    Ok(comment_id)
}

pub async fn delete_comment(pool: &PgPool, comment_id: Uuid) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM comments
        WHERE id = $1
        "#,
        comment_id,
    )
    .execute(pool)
    .await?;

    Ok(comment_id)
}
