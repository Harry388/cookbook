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

#[derive(Object)]
pub struct Section {
    title: String
}

#[derive(Object)]
pub struct SectionResult {
    pub id: i64,
    title: String,
    position: i64
}

struct PositionResult {
    position: i64
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

pub async fn add_recipe(pool: &MySqlPool, section_id: i64, recipe_id: i64, position: i64) -> Result<()> {
    sqlx::query!(
        "insert into cookbook_recipe (section_id, recipe_id, position) values (?,?,?)",
        section_id, recipe_id, position)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn remove_recipe(pool: &MySqlPool, section_id: i64, recipe_id: i64) -> Result<Option<()>> {
    let position: Option<PositionResult> = sqlx::query_as!(PositionResult,
        "select position from cookbook_recipe where section_id = ? and recipe_id = ?",
        section_id, recipe_id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    if let None = position {
        return Ok(None)
    }
    let position = position.unwrap().position;
    sqlx::query!(
        "delete from cookbook_recipe where section_id = ? and recipe_id = ?",
        section_id, recipe_id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    sqlx::query!(
        "update cookbook_recipe set position = position - 1 where position > ?",
        position)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(Some(()))
}

pub async fn add_section(pool: &MySqlPool, id: i64, section: Section, position: i64) -> Result<()> {
    sqlx::query!(
        "insert into cookbook_section (cookbook_id, title, position) values (?,?,?)",
        id, section.title, position)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn remove_section(pool: &MySqlPool, id: i64) -> Result<Option<()>> {
    let position: Option<PositionResult> = sqlx::query_as!(PositionResult,
        "select position from cookbook_section where id = ?",
        id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    if let None = position {
        return Ok(None)
    }
    let position = position.unwrap().position;
    sqlx::query!(
        "delete from cookbook_section where id = ?",
        id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    sqlx::query!(
        "update cookbook_section set position = position - 1 where position > ?",
        position)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(Some(()))
}

pub async fn get_sections(pool: &MySqlPool, id: i64) -> Result<Vec<SectionResult>> {
    let sections: Vec<SectionResult> = sqlx::query_as!(SectionResult,
        "select id, title, position from cookbook_section where cookbook_id = ? order by position asc",
        id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(sections)
}
