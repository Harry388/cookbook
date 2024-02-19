use poem::{error::InternalServerError, Result};
use sqlx::MySqlPool;

pub async fn like_comment(pool: &MySqlPool, comment_id: i64, auth: i64) -> Result<()> {
    sqlx::query!(
        "insert into comment_like (comment_id, user_id) values (?,?)",
        comment_id, auth)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn like_post(pool: &MySqlPool, post_id: i64, auth: i64) -> Result<()> {
    sqlx::query!(
        "insert into post_like (post_id, user_id) values (?,?)",
        post_id, auth)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn like_recipe(pool: &MySqlPool, recipe_id: i64, auth: i64) -> Result<()> {
    sqlx::query!(
        "insert into recipe_like (recipe_id, user_id) values (?,?)",
        recipe_id, auth)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn unlike_comment(pool: &MySqlPool, comment_id: i64, auth: i64) -> Result<()> {
    sqlx::query!(
        "delete from comment_like where comment_id = ? and user_id = ?",
        comment_id, auth)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn unlike_post(pool: &MySqlPool, post_id: i64, auth: i64) -> Result<()> {
    sqlx::query!(
        "delete from post_like where post_id = ? and user_id = ?",
        post_id, auth)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn unlike_recipe(pool: &MySqlPool, recipe_id: i64, auth: i64) -> Result<()> {
    sqlx::query!(
        "delete from recipe_like where recipe_id = ? and user_id = ?",
        recipe_id, auth)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}
