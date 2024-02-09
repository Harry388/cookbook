use poem_openapi::{payload::{Attachment, AttachmentType}, Object, Multipart, types::multipart::{Upload, JsonField}};
use poem::{error::InternalServerError, Result};
use sqlx::{MySqlPool, types::chrono::{DateTime, Utc}};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::storage::Storage;

// Inputs

#[derive(Object)]
pub struct Post {
    title: Option<String>,
    content: Option<String>,
    community_id: Option<i64>
}

#[derive(Multipart)]
pub struct PostPayload {
    post: JsonField<Post>,
    media: Vec<Upload>
}

#[derive(Object)]
pub struct UpdatePost {
    title: Option<String>,
    content: Option<String>
}

// Results

struct PartialPostResult {
    id: i64,
    title: String,
    content: Option<String>,
    user_id: i64,
    media: Option<String>,
    community_id: Option<i32>,
    created: DateTime<Utc>
}

#[derive(Object)]
pub struct PostResult {
    id: i64,
    title: String,
    content: Option<String>,
    pub user_id: i64,
    media: Vec<i64>,
    community_id: Option<i32>,
    created: DateTime<Utc>
}

struct PartialPostMediaResult {
    user_id: i64,
    uri: String
}

pub struct PostMediaResult {
    pub user_id: i64, 
    pub attachment: Attachment<Vec<u8>>
}

pub async fn create_post(pool: &MySqlPool, storage: &dyn Storage, post_payload: PostPayload, auth: i64) -> Result<()> {
    let post = post_payload.post.0;
    let post_id = sqlx::query!( 
        "insert into post (title, content, user_id, community_id) 
        values (?,?,?,?)",
        post.title, post.content, auth, post.community_id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?
        .last_insert_id();
    for media in post_payload.media {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string();
        let path = format!("user/{}/post/{}/{}", auth, post_id, time);
        let media_path = storage.put_file(&path, media).await?;
        sqlx::query!( 
            "insert into post_media (uri, post_id)
            values (?,?)",
            media_path, post_id)
            .execute(pool)
            .await
            .map_err(InternalServerError)?;
        }
    Ok(())
}

pub async fn get_post(pool: &MySqlPool, id: i64) -> Result<Option<PostResult>> {
    let post = sqlx::query_as!(PartialPostResult,
        "select post.id, post.title, post.content, post.user_id, group_concat(post_media.id) as media, created, community_id
        from post left join post_media on post.id = post_media.post_id
        where post.id = ?
        group by post.id",
        id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    if let None = post {
        return Ok(None);
    }
    let post: PartialPostResult = post.unwrap();
    let media = match post.media {
        Some(media_ids) => media_ids.split(",").map(|m| m.parse().unwrap()).collect(),
        None => vec![]
    };
    let post = PostResult { id: post.id, title: post.title, content: post.content, user_id: post.user_id, media, created: post.created, community_id: post.community_id };
    Ok(Some(post))
}

pub async fn get_post_media(pool: &MySqlPool, storage: &dyn Storage, media_id: i64) -> Result<Option<PostMediaResult>> {
    let post_media = sqlx::query_as!(PartialPostMediaResult,
        "select user_id, uri from post_media inner join post on post.id = post_media.post_id where post_media.id = ?",
        media_id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    if let None = post_media {
        return Ok(None)
    }
    let post_media: PartialPostMediaResult = post_media.unwrap();
    let media = storage.get_file(&post_media.uri).await?;
    let attachment = 
        Attachment::new(media)
        .attachment_type(AttachmentType::Inline)
        .filename(post_media.uri);
    let post_media = PostMediaResult { user_id: post_media.user_id, attachment };
    Ok(Some(post_media))
}

pub async fn get_user_posts(pool: &MySqlPool, user_id: i64) -> Result<Vec<PostResult>> {
    let posts: Vec<PartialPostResult> = sqlx::query_as!(PartialPostResult,
        "select post.id, post.title, post.content, post.user_id, group_concat(post_media.id) as media, created, community_id
        from post left join post_media on post.id = post_media.post_id
        where post.user_id = ?
        group by post.id",
        user_id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    let mut full_posts = vec![];
    for post in posts {
        let media = match post.media {
            Some(media_ids) => media_ids.split(",").map(|m| m.parse().unwrap()).collect(),
            None => vec![]
        };
        full_posts.push(PostResult { id: post.id, title: post.title, content: post.content, user_id: post.user_id, media, created: post.created, community_id: post.community_id })
    }
    Ok(full_posts)
}

pub async fn update_post(pool: &MySqlPool, id: i64, update_post: UpdatePost) -> Result<()> {
    sqlx::query!(
        "update post set title = coalesce(?, title), content = coalesce(?, content) where id = ?",
        update_post.title, update_post.content, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn delete_post(pool: &MySqlPool, storage: &dyn Storage, id: i64, auth: i64) -> Result<()> {
    sqlx::query!(
        "delete from post where id = ?",
        id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    storage.delete_file(&format!("user/{}/post/{}", auth, id)).await?;
    Ok(())
}

pub async fn add_post_recipe(pool: &MySqlPool, id: i64, recipe_id: i64) -> Result<()> {
    sqlx::query!(
        "insert into recipe_post (recipe_id, post_id) values (?,?)",
        recipe_id, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn remove_post_recipe(pool: &MySqlPool, id: i64, recipe_id: i64) -> Result<()> {
    sqlx::query!(
        "delete from recipe_post where recipe_id = ? and post_id = ?",
        recipe_id, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}
