use poem_openapi::Object;
use poem::{error::InternalServerError, Result};
use sqlx::MySqlPool;

#[derive(Object)]
pub struct Cookbook {
    title: String,
    description: Option<String>
}

#[derive(Object)]
pub struct CookbookResult {
    id: i64,
    title: String,
    description: Option<String>,
    user_id: i64
}

pub async fn create_cookbook(pool: &MySqlPool, cookbook: Cookbook, auth: i64) -> Result<()> {
    sqlx::query!(
        "insert into cookbook (title, description, user_id) values (?,?,?)",
        cookbook.title, cookbook.description, auth)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn get_cookbook(pool: &MySqlPool, id: i64) -> Result<Option<CookbookResult>> {
    let cookbook = sqlx::query_as!(CookbookResult,
        "select id, title, description, user_id from cookbook where id = ?",
        id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(cookbook)
}

pub async fn get_user_cookbooks(pool: &MySqlPool, user_id: i64) -> Result<Vec<CookbookResult>> {
    let cookbooks = sqlx::query_as!(CookbookResult,
        "select id, title, description, user_id from cookbook where user_id = ?",
        user_id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(cookbooks)
}

pub async fn update_cookbook(pool: &MySqlPool, id: i64, update: Cookbook) -> Result<()> {
    sqlx::query!(
        "update cookbook set title = coalesce(?, title), description = coalesce(?, description) where id = ?",
        update.title, update.description, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn delete_cookbook(pool: &MySqlPool, id: i64) -> Result<()> {
    sqlx::query!(
        "delete from cookbook where id = ?",
        id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}
