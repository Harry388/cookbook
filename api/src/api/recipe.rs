use poem_openapi::{OpenApi, payload::{Json, PlainText}, Object, ApiResponse, param::Path, Tags};
use poem::{web::Data, error::InternalServerError, Result};
use sqlx::{MySqlPool, types::{chrono::{DateTime, Utc}, JsonValue}};
use crate::api::auth::JWTAuthorization;
use crate::permission;

#[derive(Tags)]
enum ApiTags {
    Recipe
}

// Inputs

#[derive(Object)]
struct Recipe {
    title: String,
    description: Option<String>,
    ingredients: JsonValue,
    method: JsonValue
}

#[derive(Object)]
struct UpdateRecipe {
    title: Option<String>,
    description: Option<String>,
    ingredients: Option<JsonValue>,
    method: Option<JsonValue>
}

// Results

#[derive(Object)]
pub struct RecipeResult {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub ingredients: JsonValue,
    pub method: JsonValue,
    pub user_id: i64,
    pub created: DateTime<Utc>
}

struct CheckRecipeResult {
    user_id: i64
}

// Responses

#[derive(ApiResponse)]
enum GetRecipeResponse {
    #[oai(status = 200)]
    Ok(Json<RecipeResult>),
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

#[derive(ApiResponse)]
enum GetUserRecipesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<RecipeResult>>)
}

#[derive(ApiResponse)]
enum UpdateRecipeResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

#[derive(ApiResponse)]
enum DeleteRecipeResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

pub struct RecipeApi;

#[OpenApi(prefix_path = "/recipe", tag = "ApiTags::Recipe")]
impl RecipeApi {
    
    #[oai(path = "/", method = "post")]
    async fn create_recipe(&self, pool: Data<&MySqlPool>, recipe: Json<Recipe>, auth: JWTAuthorization) -> Result<()> {
        sqlx::query!(
            "insert into recipe (title, description, ingredients, method, user_id)
            values (?,?,?,?,?)",
            recipe.title, recipe.description, recipe.ingredients, recipe.method, auth.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetRecipeResponse> {
        let recipe: Option<RecipeResult> = sqlx::query_as!(RecipeResult,
            "select id, title, description, ingredients, method, user_id, created from recipe where id = ?",
            id.0
            )
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?;
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
        let recipes: Vec<RecipeResult> = sqlx::query_as!(RecipeResult,
            "select id, title, description, ingredients, method, user_id, created from recipe where user_id = ?",
            user_id.0
            )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(GetUserRecipesResponse::Ok(Json(recipes)))
    }

    #[oai(path = "/:id", method = "put")]
    async fn update_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, update_recipe: Json<UpdateRecipe>, auth: JWTAuthorization) -> Result<UpdateRecipeResponse> {
        let check_recipe: Option<CheckRecipeResult> = sqlx::query_as!(CheckRecipeResult,
            "select user_id from recipe where id = ?",
            id.0
            )
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?;
        if let None = check_recipe {
            return Ok(UpdateRecipeResponse::NotFound(PlainText("Recipe not found".to_string())));
        }
        let check_recipe = check_recipe.unwrap();
        permission::user::is_user(check_recipe.user_id, auth)?;
        sqlx::query!(
            "update recipe set title = coalesce(?, title), description = coalesce(?, description),
            ingredients = coalesce(?, ingredients), method = coalesce(?, method)
            where id = ?",
            update_recipe.title, update_recipe.description, update_recipe.ingredients, update_recipe.method, id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(UpdateRecipeResponse::Ok)
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<DeleteRecipeResponse> {
        let check_recipe: Option<CheckRecipeResult> = sqlx::query_as!(CheckRecipeResult,
            "select user_id from recipe where id = ?",
            id.0
            )
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?;
        if let None = check_recipe {
            return Ok(DeleteRecipeResponse::NotFound(PlainText("Recipe not found".to_string())));
        }
        let check_recipe = check_recipe.unwrap();
        permission::user::is_user(check_recipe.user_id, auth)?;
        sqlx::query!(
            "delete from recipe where id = ?",
            id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(DeleteRecipeResponse::Ok)
    }

}