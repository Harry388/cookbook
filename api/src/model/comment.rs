use poem_openapi::Object;
use poem::{error::InternalServerError, Result};
use sqlx::{MySqlPool, types::chrono::{DateTime, Utc}};

// Inputs

#[derive(Object)]
pub struct Comment {
    content: String,
    reply_id: Option<i64>
}

// Results

#[derive(Object)]
pub struct CommentResult {
    id: i64,
    content: String,
    user_id: i64,
    reply_id: Option<i32>
}

pub async fn create_comment(pool: &MySqlPool, comment: Comment, auth: i64) -> Result<u64> {
    let id = sqlx::query!(
        "insert into comment (content, user_id, reply_id) values (?,?,?)",
        comment.content, auth, comment.reply_id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?
        .last_insert_id();
    Ok(id)
}

pub async fn get_comment(pool: &MySqlPool, id: i64) -> Result<Option<CommentResult>> {
    let comment = sqlx::query_as!(CommentResult,
        "select id, content, user_id, reply_id
        from comment
        where id = ?",
        id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(comment)
}
