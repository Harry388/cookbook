use poem_openapi::{payload::{Attachment, AttachmentType}, Object, types::multipart::Upload};
use poem::{error::InternalServerError, Result};
use sqlx::{MySqlPool, types::{JsonValue, chrono::{DateTime, Utc}}};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::storage::Storage;

// Inputs

#[derive(Object)]
pub struct Post {
    #[oai(validator(max_length=255, min_length=1))]
    title: Option<String>,
    #[oai(validator(max_length=65535))]
    content: Option<String>,
    pub community_id: Option<i64>,
    recipes: Vec<i64>
}

pub type Media = Vec<Upload>;

#[derive(Object)]
pub struct UpdatePost {
    #[oai(validator(max_length=255, min_length=1))]
    title: Option<String>,
    #[oai(validator(max_length=65535))]
    content: Option<String>
}

// Results

#[derive(Object)]
pub struct PostResult {
    id: i64,
    title: String,
    content: Option<String>,
    pub user_id: i64,
    user_display_name: String,
    media: JsonValue,
    community_id: Option<i32>,
    community_title: Option<String>,
    pub created: DateTime<Utc>,
    is_liked: i64,
    pub likes: Option<i64>,
    comments: Option<i64>,
    links: Option<i64>
}

struct PartialPostMediaResult {
    post_id: i64,
    uri: String
}

pub struct PostMediaResult {
    pub post_id: i64,
    pub attachment: Attachment<Vec<u8>>
}

pub async fn create_post(pool: &MySqlPool, post: Post, auth: i64) -> Result<u64> {
    let post_id = sqlx::query!( 
        "insert into post (title, content, user_id, community_id) 
        values (?,?,?,?)",
        post.title, post.content, auth, post.community_id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?
        .last_insert_id();
    for recipe_id in post.recipes {
        add_post_recipe(pool, post_id as i64, recipe_id).await?;
    }
    Ok(post_id)
}

pub async fn add_post_post_media(pool: &MySqlPool, storage: &dyn Storage, id: i64, media_list: Media, auth: i64) -> Result<()> {
    for media in media_list {
        let content_type = media.content_type().map(|ct| ct.split("/").next().map(|ct| String::from(ct))).flatten();
        if let Some(content_type) = content_type {
            let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string();
            let path = format!("user/{}/post/{}/{}", auth, id, time);
            let media_path = storage.put_file(&path, media).await?;
            sqlx::query!(
                "insert into post_media (uri, post_id, type)
                values (?,?,?)",
                media_path, id, content_type)
                .execute(pool)
                .await
                .map_err(InternalServerError)?;
        }
    }
    Ok(())
}

pub async fn get_post(pool: &MySqlPool, id: i64, auth: i64) -> Result<Option<PostResult>> {
    let post = sqlx::query_as!(PostResult,
        "select post.id, post.title, post.content, post.user_id, json_objectagg(ifnull(post_media.id, -1), post_media.type) as media, post.created, community_id,
        user.display_name as user_display_name, community.title as community_title,
        exists (select * from post_like where post_id = post.id and user_id = ?) as is_liked,
        (select count(*) from post_like where post_id = post.id) as likes,
        (select count(*) from post_comment where post_id = post.id) as comments,
        (select count(*) from recipe_post where post_id = post.id) as links
        from post left join post_media on post.id = post_media.post_id
        inner join user on user.id = post.user_id
        left join community on community.id = post.community_id
        where post.id = ?
        group by post.id",
        auth, id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(post)
}

pub async fn get_post_media(pool: &MySqlPool, storage: &dyn Storage, media_id: i64) -> Result<Option<PostMediaResult>> {
    let post_media = sqlx::query_as!(PartialPostMediaResult,
        "select uri, post_id from post_media inner join post on post.id = post_media.post_id where post_media.id = ?",
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
    let post_media = PostMediaResult { post_id: post_media.post_id, attachment };
    Ok(Some(post_media))
}

pub async fn get_trending_posts(pool: &MySqlPool, auth: i64) -> Result<Vec<PostResult>> {
    let posts: Vec<PostResult> = sqlx::query_as!(PostResult,
        "select post.id, post.title, post.content, post.user_id, json_objectagg(ifnull(post_media.id, -1), post_media.type) as media, post.created, post.community_id,
        user.display_name as user_display_name, community.title as community_title,
        exists (select * from post_like where post_id = post.id and user_id = ?) as is_liked,
        (select count(*) from post_like where post_id = post.id) as likes,
        (select count(*) from post_comment where post_id = post.id) as comments,
        (select count(*) from recipe_post where post_id = post.id) as links
        from post left join post_media on post.id = post_media.post_id
        inner join user on user.id = post.user_id
        left join community on community.id = post.community_id
        left join following on following.following_id = post.user_id
        left join community_user on community_user.community_id = post.community_id
        inner join post_like on post_like.post_id = post.id
        where (post.user_id != ?) and ((user.public or (following.user_id = ? and following.accepted)) or (community.public or (community_user.user_id = ? and community_user.accepted))) and 
        (post_like.created > (now() - interval 1 month))
        group by post.id
        order by likes desc",
        auth, auth, auth, auth)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(posts)
}

pub async fn search_posts(pool: &MySqlPool, search: String, auth: i64) -> Result<Vec<PostResult>> {
    let posts: Vec<PostResult> = sqlx::query_as!(PostResult,
        "select post.id, post.title, post.content, post.user_id, json_objectagg(ifnull(post_media.id, -1), post_media.type) as media, post.created, post.community_id,
        user.display_name as user_display_name, community.title as community_title,
        exists (select * from post_like where post_id = post.id and user_id = ?) as is_liked,
        (select count(*) from post_like where post_id = post.id) as likes,
        (select count(*) from post_comment where post_id = post.id) as comments,
        (select count(*) from recipe_post where post_id = post.id) as links
        from post left join post_media on post.id = post_media.post_id
        inner join user on user.id = post.user_id
        left join community on community.id = post.community_id
        left join following on following.following_id = post.user_id
        left join community_user on community_user.community_id = post.community_id
        where ((post.user_id = ?) or (user.public or (following.user_id = ? and following.accepted)) or (community.public or (community_user.user_id = ? and community_user.accepted))) and 
        (match (post.title, content) against (?) or 
        exists (select * from tag inner join tag_post on tag.id = tag_post.tag_id where tag_post.post_id = post.id and match(tag.tag) against (?)))
        group by post.id
        order by created desc",
        auth, auth, auth, auth, search, search)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(posts)
}

pub async fn get_feed_posts(pool: &MySqlPool, auth: i64) -> Result<Vec<PostResult>> {
    let posts: Vec<PostResult> = sqlx::query_as!(PostResult,
        "select post.id, post.title, post.content, post.user_id, json_objectagg(ifnull(post_media.id, -1), post_media.type) as media, post.created, post.community_id,
        user.display_name as user_display_name, community.title as community_title,
        exists (select * from post_like where post_id = post.id and user_id = ?) as is_liked,
        (select count(*) from post_like where post_id = post.id) as likes,
        (select count(*) from post_comment where post_id = post.id) as comments,
        (select count(*) from recipe_post where post_id = post.id) as links
        from post left join post_media on post.id = post_media.post_id
        inner join user on user.id = post.user_id
        left join community on community.id = post.community_id
        left join following on following.following_id = post.user_id
        left join community_user on community_user.community_id = post.community_id
        left join tag_post on tag_post.post_id = post.id
        left join tag_user on tag_post.tag_id = tag_user.tag_id
        where post.user_id != ? and ((following.user_id = ? and following.accepted) or community_user.user_id = ? or ((user.public or community.public) and tag_user.user_id = ?))
        group by post.id
        order by created desc",
        auth, auth, auth, auth, auth)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(posts)
}

pub async fn get_user_posts(pool: &MySqlPool, user_id: i64, auth: i64) -> Result<Vec<PostResult>> {
    let posts: Vec<PostResult> = sqlx::query_as!(PostResult,
        "select post.id, post.title, post.content, post.user_id, json_objectagg(ifnull(post_media.id, -1), post_media.type) as media, post.created, community_id,
        user.display_name as user_display_name, community.title as community_title,
        exists (select * from post_like where post_id = post.id and user_id = ?) as is_liked,
        (select count(*) from post_like where post_id = post.id) as likes,
        (select count(*) from post_comment where post_id = post.id) as comments,
        (select count(*) from recipe_post where post_id = post.id) as links
        from post left join post_media on post.id = post_media.post_id
        inner join user on user.id = post.user_id
        left join community on community.id = post.community_id
        where post.user_id = ?
        group by post.id
        order by created desc",
        auth, user_id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(posts)
}

pub async fn get_recipe_posts(pool: &MySqlPool, id: i64, auth: i64) -> Result<Vec<PostResult>> {
    let posts: Vec<PostResult> = sqlx::query_as!(PostResult,
        "select post.id, post.title, post.content, post.user_id, json_objectagg(ifnull(post_media.id, -1), post_media.type) as media, post.created, post.community_id,
        user.display_name as user_display_name, community.title as community_title,
        exists (select * from post_like where post_id = post.id and user_id = ?) as is_liked,
        (select count(*) from post_like where post_id = post.id) as likes,
        (select count(*) from post_comment where post_id = post.id) as comments,
        (select count(*) from recipe_post where post_id = post.id) as links
        from post
        left join post_media on post.id = post_media.post_id
        inner join recipe_post on post.id = recipe_post.post_id
        inner join user on user.id = post.user_id
        left join community on community.id = post.community_id
        where recipe_post.recipe_id = ?
        group by post.id
        order by created desc",
        auth, id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(posts)
}

pub async fn get_community_posts(pool: &MySqlPool, id: i64, auth: i64) -> Result<Vec<PostResult>> {
    let posts: Vec<PostResult> = sqlx::query_as!(PostResult,
        "select post.id, post.title, post.content, post.user_id, json_objectagg(ifnull(post_media.id, -1), post_media.type) as media, post.created, post.community_id,
        user.display_name as user_display_name, community.title as community_title,
        exists (select * from post_like where post_id = post.id and user_id = ?) as is_liked,
        (select count(*) from post_like where post_id = post.id) as likes,
        (select count(*) from post_comment where post_id = post.id) as comments,
        (select count(*) from recipe_post where post_id = post.id) as links
        from post
        left join post_media on post.id = post_media.post_id
        inner join user on user.id = post.user_id
        left join community on community.id = post.community_id
        where post.community_id = ?
        group by post.id
        order by created desc",
        auth, id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(posts)
}

pub async fn get_album_posts(pool: &MySqlPool, id: i64, auth: i64) -> Result<Vec<PostResult>> {
    let posts: Vec<PostResult> = sqlx::query_as!(PostResult,
        "select post.id, post.title, post.content, post.user_id, json_objectagg(ifnull(post_media.id, -1), post_media.type) as media, post.created, post.community_id,
        user.display_name as user_display_name, community.title as community_title,
        exists (select * from post_like where post_id = post.id and user_id = ?) as is_liked,
        (select count(*) from post_like where post_id = post.id) as likes,
        (select count(*) from post_comment where post_id = post.id) as comments,
        (select count(*) from recipe_post where post_id = post.id) as links
        from post
        left join post_media on post.id = post_media.post_id
        inner join album_post on post.id = album_post.post_id
        inner join user on user.id = post.user_id
        left join community on community.id = post.community_id
        left join following on following.following_id = post.user_id
        left join community_user on community_user.community_id = post.community_id
        where (album_post.album_id = ?) and ((user.public or (following.user_id = ? and following.accepted)) or (community.public or (community_user.user_id = ? and community_user.accepted)))
        group by post.id
        order by created desc",
        auth, id, auth, auth)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(posts)
}

pub async fn get_tag_posts(pool: &MySqlPool, id: i64, auth: i64) -> Result<Vec<PostResult>> {
    let posts: Vec<PostResult> = sqlx::query_as!(PostResult,
        "select post.id, post.title, post.content, post.user_id, json_objectagg(ifnull(post_media.id, -1), post_media.type) as media, post.created, post.community_id,
        user.display_name as user_display_name, community.title as community_title,
        exists (select * from post_like where post_id = post.id and user_id = ?) as is_liked,
        (select count(*) from post_like where post_id = post.id) as likes,
        (select count(*) from post_comment where post_id = post.id) as comments,
        (select count(*) from recipe_post where post_id = post.id) as links
        from post
        left join post_media on post.id = post_media.post_id
        inner join tag_post on post.id = tag_post.post_id
        inner join user on user.id = post.user_id
        left join community on community.id = post.community_id
        left join following on following.following_id = post.user_id
        left join community_user on community_user.community_id = post.community_id
        where (tag_post.tag_id = ?) and ((user.public or (following.user_id = ? and following.accepted)) or (community.public or (community_user.user_id = ? and community_user.accepted)))
        group by post.id
        order by created desc",
        auth, id, auth, auth)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(posts)
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

pub async fn add_album_post(pool: &MySqlPool, id: i64, album_id: i64) -> Result<()> {
    sqlx::query!(
        "insert into album_post (album_id, post_id) values (?,?)",
        album_id, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn remove_album_post(pool: &MySqlPool, id: i64, album_id: i64) -> Result<()> {
    sqlx::query!(
        "delete from album_post where album_id = ? and post_id = ?",
        album_id, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn add_post_tags(pool: &MySqlPool, id: i64, tag_ids: Vec<i64>) -> Result<()> {
    for tag_id in tag_ids.iter() {
        sqlx::query!(
            "insert into tag_post (tag_id, post_id) values (?,?)",
            tag_id, id) 
            .execute(pool)
            .await
            .map_err(InternalServerError)?;
    }
    Ok(())
}

pub async fn remove_post_tags(pool: &MySqlPool, id: i64, tag_ids: Vec<i64>) -> Result<()> {
    for tag_id in tag_ids.iter() {
        sqlx::query!(
            "delete from tag_post where tag_id = ? and post_id = ?",
            tag_id, id) 
            .execute(pool)
            .await
            .map_err(InternalServerError)?;
    }
    Ok(())
}

pub async fn remove_community(pool: &MySqlPool, id: i64) -> Result<()> {
    sqlx::query!(
            "update post set community_id = null where id = ?",
            id)
            .execute(pool)
            .await
            .map_err(InternalServerError)?;
    Ok(())
}
