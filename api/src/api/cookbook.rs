use poem_openapi::{OpenApi, payload::Json, param::Path, Tags, ApiResponse};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::model::cookbook;

#[derive(Tags)]
enum ApiTags {
    Cookbook
}

// Responses

#[derive(ApiResponse)]
enum GetCookbookResponse {
    #[oai(status = 200)]
    Ok(Json<cookbook::CookbookResult>),
    #[oai(status = 404)]
    NotFound
}

#[derive(ApiResponse)]
enum GetUserCookbookResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<cookbook::CookbookResult>>)
}

pub struct CookbookApi;

#[OpenApi(prefix_path = "/cookbook", tag = "ApiTags::Cookbook")]
impl CookbookApi {

    #[oai(path = "/", method = "post")]
    async fn create_cookbook(&self, pool: Data<&MySqlPool>, cookbook: Json<cookbook::Cookbook>, auth: JWTAuthorization) -> Result<()> {
        cookbook::create_cookbook(pool.0, cookbook.0, auth.0).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_cookbook(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetCookbookResponse> {
        permission::cookbook::is_visible(pool.0, id.0, auth).await?;
        let cookbook = cookbook::get_cookbook(pool.0, id.0).await?;
        Ok(
            match cookbook {
                Some(cb) => GetCookbookResponse::Ok(Json(cb)),
                None => GetCookbookResponse::NotFound
            }
        )
    }

    #[oai(path = "/user/:user_id", method = "get")]
    async fn get_user_cookbooks(&self, pool: Data<&MySqlPool>, user_id: Path<i64>, auth: JWTAuthorization) -> Result<GetUserCookbookResponse> {
        permission::user::is_following_or_public(pool.0, user_id.0, auth).await?;
        let cookbooks = cookbook::get_user_cookbooks(pool.0, user_id.0).await?;
        Ok(GetUserCookbookResponse::Ok(Json(cookbooks)))
    }

    #[oai(path = "/:id", method = "put")]
    async fn update_cookbook(&self, pool: Data<&MySqlPool>, id: Path<i64>, cookbook: Json<cookbook::Cookbook>, auth: JWTAuthorization) -> Result<()> {
        permission::cookbook::owns_cookbook(pool.0, id.0, auth).await?;
        cookbook::update_cookbook(pool.0, id.0, cookbook.0).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_cookbook(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::cookbook::owns_cookbook(pool.0, id.0, auth).await?;
        cookbook::delete_cookbook(pool.0, id.0).await?;
        Ok(())
    }

}
