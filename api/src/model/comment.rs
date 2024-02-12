use poem_openapi::Object;
use poem::{error::InternalServerError, Result};
use sqlx::{MySqlPool, types::chrono::{DateTime, Utc}};

// Inputs

#[derive(Object)]
pub struct Comment {
    content: String,
    reply_id: Option<i64>
}

#[derive(Object)]
pub struct UpdateComment {
    content: String
}

// Results

#[derive(Object)]
pub struct CommentResult {
    id: i64,
    content: String,
    user_id: i64,
    reply_id: Option<i32>
}

async fn create_comment(pool: &MySqlPool, comment: Comment, auth: i64) -> Result<u64> {
    let id = sqlx::query!(
        "insert into comment (content, user_id, reply_id) values (?,?,?)",
        comment.content, auth, comment.reply_id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?
        .last_insert_id();
    Ok(id)
}

pub async fn create_post_comment(pool: &MySqlPool, comment: Comment, post_id: i64, auth: i64) -> Result<()> {
    let comment_id = create_comment(pool, comment, auth).await?;
    sqlx::query!(
        "insert into post_comment (post_id, comment_id) values (?,?)",
        post_id, comment_id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn create_recipe_comment(pool: &MySqlPool, comment: Comment, recipe_id: i64, auth: i64) -> Result<()> {
    let comment_id = create_comment(pool, comment, auth).await?;
    sqlx::query!(
        "insert into recipe_comment (recipe_id, comment_id) values (?,?)",
        recipe_id, comment_id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn get_post_comments(pool: &MySqlPool, post_id: i64) -> Result<Vec<CommentResult>> {
    let comments = sqlx::query_as!(CommentResult,
        "select id, content, user_id, reply_id
        from comment inner join post_comment on comment.id = post_comment.comment_id
        where post_comment.post_id = ?",
        post_id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(comments)
}

pub async fn get_recipe_comments(pool: &MySqlPool, recipe_id: i64) -> Result<Vec<CommentResult>> {
    let comments = sqlx::query_as!(CommentResult,
        "select id, content, user_id, reply_id
        from comment inner join recipe_comment on comment.id = recipe_comment.comment_id
        where recipe_comment.recipe_id = ?",
        recipe_id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(comments)
}

pub async fn update_comment(pool: &MySqlPool, id: i64, update: UpdateComment) -> Result<()> {
    sqlx::query!(
        "update comment set content = coalesce(?, content)
        where id = ?",
        update.content, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn delete_comment(pool: &MySqlPool, id: i64) -> Result<()> {
    sqlx::query!(
        "delete from comment where id = ?",
        id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}
