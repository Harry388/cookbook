use poem_openapi::{OpenApi, payload::Json, Object, ApiResponse, Tags, param::Path};
use poem::{web::Data, error::InternalServerError, Result};
use sqlx::MySqlPool;
use crate::api::auth::JWTAuthorization;

#[derive(Tags)]
enum ApiTags {
    Tag
}

// Inputs

#[derive(Object)]
struct Tag {
    tag: String
}

// Results

#[derive(Object)]
struct TagResult {
    id: i64,
    tag: String
}

// Responses

#[derive(ApiResponse)]
enum GetAllTagsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<TagResult>>)
}

pub struct TagApi;

#[OpenApi(prefix_path = "/tag", tag = "ApiTags::Tag")]
impl TagApi {

    #[oai(path = "/", method = "post")]
    async fn create_tag(&self, pool: Data<&MySqlPool>, tag: Json<Tag>, _auth: JWTAuthorization) -> Result<()> {
        sqlx::query!(
            "insert into tag (tag) values (?)",
            tag.tag
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/", method = "get")]
    async fn get_all_tags(&self, pool: Data<&MySqlPool>, _auth: JWTAuthorization) -> Result<GetAllTagsResponse> {
        let tags = sqlx::query_as!(TagResult,
            "select id, tag from tag",
            )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;
       Ok(GetAllTagsResponse::Ok(Json(tags))) 
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_tag(&self, pool: Data<&MySqlPool>, id: Path<i64>, _auth: JWTAuthorization) -> Result<()> {
        sqlx::query!(
            "delete from tag where id = ?",
            id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

}
