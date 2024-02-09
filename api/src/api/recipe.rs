use poem_openapi::{OpenApi, payload::{Json, PlainText}, ApiResponse, param::Path, Tags};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::model::{post, recipe};

#[derive(Tags)]
enum ApiTags {
    Recipe
}

// Responses

#[derive(ApiResponse)]
enum GetRecipeResponse {
    #[oai(status = 200)]
    Ok(Json<recipe::RecipeResult>),
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

#[derive(ApiResponse)]
enum GetUserRecipesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<recipe::RecipeResult>>)
}

#[derive(ApiResponse)]
enum GetRecipePostsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<post::PostResult>>)
}

pub struct RecipeApi;

#[OpenApi(prefix_path = "/recipe", tag = "ApiTags::Recipe")]
impl RecipeApi {
    
    #[oai(path = "/", method = "post")]
    async fn create_recipe(&self, pool: Data<&MySqlPool>, recipe: Json<recipe::Recipe>, auth: JWTAuthorization) -> Result<()> {
        recipe::create_recipe(pool.0, recipe.0, auth.0).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetRecipeResponse> {
        let recipe = recipe::get_recipe(pool.0, id.0).await?;
        if let None = recipe {
            return Ok(GetRecipeResponse::NotFound(PlainText("Recipe not found".to_string())));
        }
        let recipe = recipe.unwrap();
        permission::user::is_following_or_public(pool.0, recipe.user_id, auth).await?;
        Ok(GetRecipeResponse::Ok(Json(recipe)))
    }

    #[oai(path = "/user/:user_id", method = "get")]
    async fn get_user_recipes(&self, pool: Data<&MySqlPool>, user_id: Path<i64>, auth: JWTAuthorization) -> Result<GetUserRecipesResponse> {
        permission::user::is_following_or_public(pool.0, user_id.0, auth).await?;
        let recipes = recipe::get_user_recipes(pool.0, user_id.0).await?;
        Ok(GetUserRecipesResponse::Ok(Json(recipes)))
    }

    #[oai(path = "/:id", method = "put")]
    async fn update_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, update_recipe: Json<recipe::UpdateRecipe>, auth: JWTAuthorization) -> Result<()> {
        permission::recipe::owns_recipe(pool.0, id.0, auth).await?;
        recipe::update_recipe(pool.0, id.0, update_recipe.0).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::recipe::owns_recipe(pool.0, id.0, auth).await?;
        recipe::delete_recipe(pool.0, id.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/post", method = "get")]
    async fn get_recipe_posts(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetRecipePostsResponse> {
        permission::recipe::is_visible(pool.0, id.0, auth).await?;
        let posts = post::get_recipe_posts(pool.0, id.0).await?;
        Ok(GetRecipePostsResponse::Ok(Json(posts)))
    }

}
