use poem_openapi::{OpenApi, payload::{Json, PlainText, Attachment}, Object, ApiResponse, param::Path, Tags, Multipart, types::multipart::{Upload, JsonField}};
use poem::{web::Data, error::InternalServerError, Result};
use sqlx::MySqlPool;
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
    media: Upload
}


// Results



// Responses



pub struct PostApi;

#[OpenApi(prefix_path = "/post", tag = "ApiTags::Post")]
impl PostApi {
    
    #[oai(path = "/", method = "post")]
    async fn create_post(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, post_payload: PostPayload, auth: JWTAuthorization) -> Result<()> {
        permission::user::is_user(post_payload.post.0.user_id, auth)?;
        storage.0.put_file("public/test.png", post_payload.media).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_post(&self) {

    }

    #[oai(path = "/user/:user_id", method = "get")]
    async fn get_user_posts(&self) {

    }

    #[oai(path = "/:id", method = "put")]
    async fn update_user(&self) {

    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_user(&self) {

    }

}