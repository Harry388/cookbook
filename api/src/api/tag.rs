use poem_openapi::{OpenApi, payload::Json, param::Path, Tags};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::model::{tag, recipe};

#[derive(Tags)]
enum ApiTags {
    Tag
}

// Responses

type GetRecipeTagsResponse = Result<Json<Vec<tag::TagResult>>>;

type GetTagEntriesResponse = Result<Json<Vec<recipe::RecipeResult>>>;


pub struct TagApi;

#[OpenApi(prefix_path = "/tag", tag = "ApiTags::Tag")]
impl TagApi {

    #[oai(path = "/recipe/:recipe_id", method = "get")]
    async fn get_recipe_tags(&self, pool: Data<&MySqlPool>, recipe_id: Path<i64>, auth: JWTAuthorization) -> GetRecipeTagsResponse {
        permission::recipe::is_visible(pool.0, recipe_id.0, auth).await?;
        let tags = tag::get_recipe_tags(pool.0, recipe_id.0).await?;
        Ok(Json(tags))
    }

    #[oai(path = "/:id/entries", method = "get")]
    async fn get_tag_entries(&self, pool: Data<&MySqlPool>, id: Path<i64>, _auth: JWTAuthorization) -> GetTagEntriesResponse {
        let recipes = recipe::get_tag_recipes(pool.0, id.0).await?;
        Ok(Json(recipes))
    }
    
}
