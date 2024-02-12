use poem_openapi::{OpenApi, payload::{Json, PlainText}, ApiResponse, param::Path, Tags};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::model::comment;

#[derive(Tags)]
enum ApiTags {
    Comment
}

// Responses

#[derive(ApiResponse)]
enum GetCommentResponse {
    #[oai(status = 200)]
    Ok(Json<comment::CommentResult>),
    #[oai(status = 404)]
    NotFound
}

pub struct CommentApi;

#[OpenApi(prefix_path = "/comment", tag = "ApiTags::Comment")]
impl CommentApi {

    #[oai(path = "/", method = "post")]
    async fn create_comment(&self, pool: Data<&MySqlPool>, comment: Json<comment::Comment>, auth: JWTAuthorization) -> Result<()> {
        comment::create_comment(pool.0, comment.0, auth.0).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_comment(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetCommentResponse> {
        let comment = comment::get_comment(pool.0, id.0).await?;
        Ok(
            match comment {
                Some(c) => GetCommentResponse::Ok(Json(c)),
                None => GetCommentResponse::NotFound
            }
        )
    }
    
}
