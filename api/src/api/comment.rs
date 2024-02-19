use poem_openapi::{OpenApi, payload::Json, param::Path, Tags};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::model::{comment, like};

#[derive(Tags)]
enum ApiTags {
    Comment
}

// Responses


pub struct CommentApi;

#[OpenApi(prefix_path = "/comment", tag = "ApiTags::Comment")]
impl CommentApi {

    #[oai(path = "/:id", method = "put")]
    async fn update_comment(&self, pool: Data<&MySqlPool>, id: Path<i64>, update: Json<comment::UpdateComment>, auth: JWTAuthorization) -> Result<()> {
        permission::comment::owns_comment(pool.0, id.0, auth).await?;
        comment::update_comment(pool.0, id.0, update.0).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_comment(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::comment::owns_comment(pool.0, id.0, auth).await?;
        comment::delete_comment(pool.0, id.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/like", method = "post")]
    async fn like_comment(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        like::like_comment(pool.0, id.0, auth.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/like", method = "delete")]
    async fn unlike_comment(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        like::unlike_comment(pool.0, id.0, auth.0).await?;
        Ok(())
    }
    
}
