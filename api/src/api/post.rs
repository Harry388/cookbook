use poem_openapi::{OpenApi, payload::{Json, PlainText, Attachment, AttachmentType}, Object, ApiResponse, param::Path, Tags, Multipart, types::multipart::{Upload, JsonField}};
use poem::{web::Data, error::InternalServerError, Result};
use sqlx::MySqlPool;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::storage::{Storage, dufs::DufsStorage};

#[derive(Tags)]
enum ApiTags {
    Post
}

// Inputs

#[derive(Object)]
struct Post {
    title: String,
    content: Option<String>
}

#[derive(Multipart)]
struct PostPayload {
    post: JsonField<Post>,
    media: Vec<Upload>
}


// Results

struct PostResult {
    id: i64,
    title: String,
    content: Option<String>,
    user_id: i64,
    media: Option<String>
}

struct PostMediaResult {
    user_id: i64,
    uri: String
}

// Responses

#[derive(Object)]
struct PostResponse {
    id: i64,
    title: String,
    content: Option<String>,
    user_id: i64,
    media: Vec<i64>
}

#[derive(ApiResponse)]
enum GetPostResponse {
    #[oai(status = 200)]
    Ok(Json<PostResponse>),
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

#[derive(ApiResponse)]
enum GetPostMediaResponse {
    #[oai(status = 200)]
    Ok(Attachment<Vec<u8>>),
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

#[derive(ApiResponse)]
enum GetUserPostsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<PostResponse>>)
}

pub struct PostApi;

#[OpenApi(prefix_path = "/post", tag = "ApiTags::Post")]
impl PostApi {
    
    #[oai(path = "/", method = "post")]
    async fn create_post(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, post_payload: PostPayload, auth: JWTAuthorization) -> Result<()> {
        let post = post_payload.post.0;
        let post_id = sqlx::query!( 
            "insert into post (title, content, user_id)
            values (?,?,?)",
            post.title, post.content, auth.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?
            .last_insert_id();
        for media in post_payload.media {
            let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string();
            let path = format!("user/{}/{}", auth.0, time);
            let media_path = storage.0.put_file(&path, media).await?;
            sqlx::query!( 
                "insert into post_media (uri, post_id)
                values (?,?)",
                media_path, post_id
                )
                .execute(pool.0)
                .await
                .map_err(InternalServerError)?;
        }
        Ok(())
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_post(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetPostResponse> {
        let post = sqlx::query_as!(PostResult,
            "select post.id, post.title, post.content, post.user_id, group_concat(post_media.id) as media
            from post inner join post_media on post.id = post_media.post_id
            where post.id = ?
            group by post.id",
            id.0
            )
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?;
        if let None = post {
            return Ok(GetPostResponse::NotFound(PlainText("Post not found".to_string())));
        }
        let post: PostResult = post.unwrap();
        permission::user::is_following_or_public(pool.0, post.user_id, auth).await?;
        let media = match post.media {
            Some(media_ids) => media_ids.split(",").map(|m| m.parse().unwrap()).collect(),
            None => vec![]
        };
        let post = PostResponse { id: post.id, title: post.title, content: post.content, user_id: post.user_id, media };
        Ok(GetPostResponse::Ok(Json(post)))
    }

    #[oai(path = "/media/:media_id", method = "get")]
    async fn get_post_media(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, media_id: Path<i64>, auth: JWTAuthorization) -> Result<GetPostMediaResponse> {
        let post_media = sqlx::query_as!(PostMediaResult,
            "select user_id, uri from post_media inner join post on post.id = post_media.post_id where post_media.id = ?",
            media_id.0
            )
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?;
        if let None = post_media {
            return Ok(GetPostMediaResponse::NotFound(PlainText("Post not found".to_string())));
        }
        let post_media: PostMediaResult = post_media.unwrap();
        permission::user::is_following_or_public(pool.0, post_media.user_id, auth).await?;
        let media = storage.0.get_file(&post_media.uri).await?;
        let attachment = 
            Attachment::new(media)
            .attachment_type(AttachmentType::Inline)
            .filename(post_media.uri);
        Ok(GetPostMediaResponse::Ok(attachment))
    }

    #[oai(path = "/user/:user_id", method = "get")]
    async fn get_user_posts(&self, pool: Data<&MySqlPool>, user_id: Path<i64>, auth: JWTAuthorization) -> Result<GetUserPostsResponse> {
        permission::user::is_following_or_public(pool.0, user_id.0, auth).await?;
        let posts: Vec<PostResult> = sqlx::query_as!(PostResult,
            "select post.id, post.title, post.content, post.user_id, group_concat(post_media.id) as media
            from post inner join post_media on post.id = post_media.post_id
            where post.user_id = ?
            group by post.id",
            user_id.0
            )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;
        let mut full_posts = vec![];
        for post in posts {
            let media = match post.media {
                Some(media_ids) => media_ids.split(",").map(|m| m.parse().unwrap()).collect(),
                None => vec![]
            };
            full_posts.push(PostResponse { id: post.id, title: post.title, content: post.content, user_id: post.user_id, media })
        }
        Ok(GetUserPostsResponse::Ok(Json(full_posts)))
    }

    #[oai(path = "/:id", method = "put")]
    async fn update_post(&self) {

    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_post(&self) {

    }

}