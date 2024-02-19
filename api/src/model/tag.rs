use poem_openapi::Object;
use poem::{error::InternalServerError, Result};
use sqlx::MySqlPool;

// Inputs

pub type Tags = Vec<String>;

// Results

struct IdResult {
    id: i64
}

#[derive(Object)]
pub struct TagResult {
    id: i64,
    tag: String
}

pub async fn create_tags(pool: &MySqlPool, tags: Tags) -> Result<Vec<i64>> {
    for tag in tags.iter() {
        let _ = sqlx::query!(
            "insert into tag (tag) values (?)",
            tag)
            .execute(pool)
            .await;
    }
    let mut ids = vec![];
    for tag in tags.iter() {
        let idr = sqlx::query_as!(IdResult,
            "select id from tag where tag = ?",
            tag)
            .fetch_optional(pool)
            .await
            .map_err(InternalServerError)?;
        if let Some(idr) = idr {
            ids.push(idr.id);
        }
    }
    Ok(ids)
}

pub async fn get_tag(pool: &MySqlPool, id: i64) -> Result<Option<TagResult>> {
    let tag = sqlx::query_as!(TagResult,
        "select id, tag from tag where id = ?",
        id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(tag)
}

pub async fn get_recipe_tags(pool: &MySqlPool, recipe_id: i64) -> Result<Vec<TagResult>> {
    let tags = sqlx::query_as!(TagResult,
        "select tag.id, tag.tag
        from tag inner join tag_entry on tag.id = tag_entry.tag_id
        where tag_entry.recipe_id = ?",
        recipe_id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(tags)
}

pub async fn get_post_tags(pool: &MySqlPool, post_id: i64) -> Result<Vec<TagResult>> {
    let tags = sqlx::query_as!(TagResult,
        "select tag.id, tag.tag
        from tag inner join tag_entry on tag.id = tag_entry.tag_id
        where tag_entry.post_id = ?",
        post_id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(tags)
}
