use poem_openapi::{OpenApi, payload::{Json, PlainText, Attachment}, Object, ApiResponse, param::Path, Tags, Multipart};
use poem::{web::Data, error::InternalServerError, Result};
use sqlx::MySqlPool;
use crate::api::auth::JWTAuthorization;
use crate::permission;

#[derive(Tags)]
enum ApiTags {
    Post
}

// Inputs

#[derive(Debug, Multipart)]
struct Post {
    title: String,
    content: Option<String>,
    user_id: i64,
    media: Vec<Upload>
}


// Results



// Responses



pub struct PostApi;

#[OpenApi(prefix_path = "/post", tag = "ApiTags::Post")]
impl PostApi {
    
    #[oai(path = "/", method = "post")]
    async fn create_post(&self, pool: Data<&MySqlPool>, post: Post, auth: JWTAuthorization) -> Result<()> {
        permission::user::is_user(post.user_id, auth)?;
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