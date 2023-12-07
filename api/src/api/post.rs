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
    content: Option<String>,
    user_id: i64
}

#[derive(Multipart)]
struct PostPayload {
    post: JsonField<Post>,
    media: Option<Upload>
}


// Results

#[derive(Object)]
struct PostResult {
    id: i64,
    title: String,
    content: Option<String>,
    user_id: i64
}

struct PostMediaResult {
    user_id: i64,
    uri: String
}

// Responses

#[derive(ApiResponse)]
enum GetPostResponse {
    #[oai(status = 200)]
    Ok(Json<PostResult>),
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
    Ok(Json<Vec<PostResult>>)
}

pub struct PostApi;

#[OpenApi(prefix_path = "/post", tag = "ApiTags::Post")]
impl PostApi {
    
    #[oai(path = "/", method = "post")]
    async fn create_post(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, post_payload: PostPayload, auth: JWTAuthorization) -> Result<()> {
        let post = post_payload.post.0;
        permission::user::is_user(post.user_id, auth)?;
        let post_id = sqlx::query!( 
            "insert into post (title, content, user_id)
            values (?,?,?)",
            post.title, post.content, post.user_id
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?
            .last_insert_id();
        if let Some(media) = post_payload.media {
            let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string();
            let path = format!("user/{}/{}", post.user_id, time);
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
            "select * from post where id = ?",
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
        Ok(GetPostResponse::Ok(Json(post)))
    }

    #[oai(path = "/:id/media", method = "get")]
    async fn get_post_media(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetPostMediaResponse> {
        let post_media = sqlx::query_as!(PostMediaResult,
            "select user_id, uri from post_media inner join post on post.id = post_media.post_id where post.id = ?",
            id.0
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
        let posts = sqlx::query_as!(PostResult,
            "select * from post where user_id = ?",
            user_id.0
            )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(GetUserPostsResponse::Ok(Json(posts)))
    }

    #[oai(path = "/:id", method = "put")]
    async fn update_post(&self) {

    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_post(&self) {

    }

}