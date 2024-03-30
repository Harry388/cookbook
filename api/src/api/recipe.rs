use poem_openapi::{OpenApi, payload::{Json, PlainText}, ApiResponse, param::Path, Tags, Object};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use futures::try_join;
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::model::{post, recipe, comment, tag, like};

#[derive(Tags)]
enum ApiTags {
    Recipe
}

// Inputs

#[derive(Object)]
struct RecipeWithTags {
    recipe: recipe::Recipe,
    tags: tag::Tags
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

#[derive(ApiResponse)]
enum GetRecipeCommentsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<comment::CommentResult>>)
}

pub struct RecipeApi;

#[OpenApi(prefix_path = "/recipe", tag = "ApiTags::Recipe")]
impl RecipeApi {
    
    #[oai(path = "/", method = "post")]
    async fn create_recipe(&self, pool: Data<&MySqlPool>, recipe: Json<RecipeWithTags>, auth: JWTAuthorization) -> Result<Json<u64>> {
        let recipe_fut = recipe::create_recipe(pool.0, recipe.0.recipe, auth.0);
        let tags_fut = tag::create_tags(pool.0, recipe.0.tags);
        let (recipe_id, tag_ids) = try_join!(recipe_fut, tags_fut)?;
        recipe::add_recipe_tags(pool.0, recipe_id as i64, tag_ids).await?;
        Ok(Json(recipe_id))
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetRecipeResponse> {
        let recipe = recipe::get_recipe(pool.0, id.0, auth.0).await?;
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
        let recipes = recipe::get_user_recipes(pool.0, user_id.0, auth.0).await?;
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
        let posts = post::get_recipe_posts(pool.0, id.0, auth.0).await?;
        Ok(GetRecipePostsResponse::Ok(Json(posts)))
    }

    #[oai(path = "/:id/comment", method = "get")]
    async fn get_recipe_comments(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetRecipeCommentsResponse> {
        permission::recipe::is_visible(pool.0, id.0, auth).await?;
        let comments = comment::get_recipe_comments(pool.0, id.0, auth.0).await?;
        Ok(GetRecipeCommentsResponse::Ok(Json(comments)))
    }

    #[oai(path = "/:id/comment", method = "post")]
    async fn create_recipe_comment(&self, pool: Data<&MySqlPool>, id: Path<i64>, comment: Json<comment::Comment>, auth: JWTAuthorization) -> Result<()> {
        permission::recipe::is_visible(pool.0, id.0, auth).await?;
        comment::create_recipe_comment(pool.0, comment.0, id.0, auth.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/like", method = "post")]
    async fn like_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::recipe::is_visible(pool.0, id.0, auth).await?;
        like::like_recipe(pool.0, id.0, auth.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/like", method = "delete")]
    async fn unlike_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::recipe::is_visible(pool.0, id.0, auth).await?;
        like::unlike_recipe(pool.0, id.0, auth.0).await?;
        Ok(())
    }

}
