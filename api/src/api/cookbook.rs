use poem_openapi::{OpenApi, payload::Json, param::Path, Tags, ApiResponse};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use futures::try_join;
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::model::{cookbook, recipe};

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

type GetCookBookRecipesResponse = Json<Vec<recipe::RecipeResult>>;

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

    #[oai(path = "/:id/recipe", method = "get")]
    async fn get_recipes(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetCookBookRecipesResponse> {
        permission::cookbook::is_visible(pool.0, id.0, auth).await?;
        let recipes = recipe::get_cookbook_recipes(pool.0, id.0, auth.0).await?;
        Ok(Json(recipes))
    }

    #[oai(path = "/:id/recipe/:recipe_id", method = "post")]
    async fn add_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, recipe_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        let owns_cookbook = permission::cookbook::owns_cookbook(pool.0, id.0, auth);
        let owns_recipe = permission::recipe::owns_recipe(pool.0, recipe_id.0, auth);
        try_join!(owns_cookbook, owns_recipe)?;
        let recipes = recipe::get_cookbook_recipes(pool.0, id.0, auth.0).await?;
        cookbook::add_recipe(pool.0, id.0, recipe_id.0, recipes.len().try_into().unwrap()).await?;
        Ok(())
    }

    #[oai(path = "/:id/recipe/:recipe_id", method = "delete")]
    async fn remove_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, recipe_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        let owns_cookbook = permission::cookbook::owns_cookbook(pool.0, id.0, auth);
        let owns_recipe = permission::recipe::owns_recipe(pool.0, recipe_id.0, auth);
        try_join!(owns_cookbook, owns_recipe)?;
        cookbook::remove_recipe(pool.0, id.0, recipe_id.0).await?;
        Ok(())
    }

}
