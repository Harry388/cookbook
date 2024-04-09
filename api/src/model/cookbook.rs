use poem_openapi::{Object, payload::{AttachmentType, Attachment}, Multipart, types::multipart::Upload};
use poem::{error::InternalServerError, Result};
use sqlx::MySqlPool;
use crate::storage::Storage;

#[derive(Object)]
pub struct Cookbook {
    #[oai(validator(max_length=255, min_length = 1))]
    title: String,
    #[oai(validator(max_length=65535))]
    description: Option<String>
}

#[derive(Object)]
pub struct CookbookResult {
    id: i64,
    title: String,
    description: Option<String>,
    user_id: i64,
    user_display_name: String
}

#[derive(Object)]
pub struct Section {
    #[oai(validator(max_length=255, min_length = 1))]
    title: String
}

#[derive(Object)]
pub struct UpdateSection {
    #[oai(validator(max_length=255))]
    title: Option<String>,
    #[oai(validator(max_length=65535))]
    description: Option<String>
}

#[derive(Object)]
pub struct SectionResult {
    pub id: i64,
    title: String,
    description: Option<String>,
    position: i64
}

struct PositionResult {
    position: i64
}

#[derive(Multipart)]
pub struct SetRecipePic {
    image: Upload
}

struct GetRecipePicResult {
    image: Option<String>
}

pub enum RecipePicResult {
    Ok(Attachment<Vec<u8>>),
    RecipeNotFound,
    PicNotFound
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
        "select cookbook.id, title, description, user_id, user.display_name as user_display_name 
        from cookbook
        inner join user on cookbook.user_id = user.id
        where cookbook.id = ?",
        id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(cookbook)
}

pub async fn get_user_cookbooks(pool: &MySqlPool, user_id: i64) -> Result<Vec<CookbookResult>> {
    let cookbooks = sqlx::query_as!(CookbookResult,
        "select cookbook.id, title, description, user_id, user.display_name as user_display_name
        from cookbook
        inner join user on cookbook.user_id = user.id
        where user_id = ?",
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

pub async fn delete_cookbook(pool: &MySqlPool, storage: &dyn Storage, id: i64) -> Result<()> {
    sqlx::query!(
        "delete from cookbook where id = ?",
        id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    let path = format!("cookbook/{}", id);
    storage.delete_file(&path).await?;
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

pub async fn remove_recipe(pool: &MySqlPool, storage: &dyn Storage, section_id: i64, recipe_id: i64) -> Result<Option<()>> {
    remove_recipe_pic(pool, storage, section_id, recipe_id).await?;
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

pub async fn update_section(pool: &MySqlPool, id: i64, update: UpdateSection) -> Result<()> {
    sqlx::query!(
        "update cookbook_section set title = coalesce(?, title), description = coalesce(?, description) where id = ?",
        update.title, update.description, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn remove_section(pool: &MySqlPool, storage: &dyn Storage, id: i64, cookbook_id: i64) -> Result<Option<()>> {
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
    let path = format!("cookbook/{}/section/{}", cookbook_id, id);
    storage.delete_file(&path).await?;
    Ok(Some(()))
}

pub async fn get_sections(pool: &MySqlPool, id: i64) -> Result<Vec<SectionResult>> {
    let sections: Vec<SectionResult> = sqlx::query_as!(SectionResult,
        "select id, title, description, position from cookbook_section where cookbook_id = ? order by position asc",
        id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(sections)
}

pub async fn set_recipe_pic(pool: &MySqlPool, storage: &dyn Storage, id: i64, section_id: i64, recipe_id: i64, pic: SetRecipePic) -> Result<()> {
    let current_pic: Option<GetRecipePicResult> = sqlx::query_as!(GetRecipePicResult,
        "select image from cookbook_recipe where section_id = ? and recipe_id = ?",
        section_id, recipe_id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    if let Some(cpic) = current_pic {
        if let Some(current_path) = cpic.image {
            storage.delete_file(&current_path).await?;
        }
    }
    let path = format!("cookbook/{}/section/{}/recipe/{}/image", id, section_id, recipe_id);
    let pic_path = storage.put_file(&path, pic.image).await?;
    sqlx::query!(
        "update cookbook_recipe set image = ? where section_id = ? and recipe_id = ?",
        pic_path, section_id, recipe_id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn get_recipe_pic(pool: &MySqlPool, storage: &dyn Storage, section_id: i64, recipe_id: i64) -> Result<RecipePicResult> {
    let pic: Option<GetRecipePicResult> = sqlx::query_as!(GetRecipePicResult,
        "select image from cookbook_recipe where section_id = ? and recipe_id = ?",
        section_id, recipe_id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    if let None = pic {
        return Ok(RecipePicResult::RecipeNotFound);
    }
    if let None = pic.as_ref().unwrap().image {
        return Ok(RecipePicResult::PicNotFound);
    }
    let pic_path = pic.unwrap().image.unwrap();
    let pic = storage.get_file(&pic_path).await?;
    let attachment = 
        Attachment::new(pic)
        .attachment_type(AttachmentType::Inline)
        .filename(pic_path);
    Ok(RecipePicResult::Ok(attachment))
}

pub async fn recipe_has_pic(pool: &MySqlPool, section_id: i64, recipe_id: i64) -> Result<bool> {
    let pic: Option<GetRecipePicResult> = sqlx::query_as!(GetRecipePicResult,
        "select image from cookbook_recipe where section_id = ? and recipe_id = ?",
        section_id, recipe_id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    if let None = pic {
        return Ok(false);
    }
    if let None = pic.as_ref().unwrap().image {
        return Ok(false);
    }
    let pic_path = pic.unwrap().image.unwrap();
    Ok(pic_path.len() > 0)
}

pub async fn remove_recipe_pic(pool: &MySqlPool, storage: &dyn Storage, section_id: i64, recipe_id: i64) -> Result<()> {
    let pic: Option<GetRecipePicResult> = sqlx::query_as!(GetRecipePicResult,
        "select image from cookbook_recipe where section_id = ? and recipe_id = ?",
        section_id, recipe_id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    if let None = pic {
        return Ok(());
    }
    if let None = pic.as_ref().unwrap().image {
        return Ok(());
    }
    let pic_path = pic.unwrap().image.unwrap();
    storage.delete_file(&pic_path).await?;
    sqlx::query!(
        "update cookbook_recipe set image = null where section_id = ? and recipe_id = ?",
        section_id, recipe_id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}
