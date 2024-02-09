use poem_openapi::{OpenApi, payload::{Json, PlainText, Attachment}, Object, ApiResponse, param::Path, Tags};
use poem::{web::Data, error::InternalServerError, Result};
use sqlx::{MySqlPool, types::chrono::{DateTime, Utc}};
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::storage::dufs::DufsStorage;
use crate::api::recipe::RecipeResult;
use crate::model::post;

#[derive(Tags)]
enum ApiTags {
    Post
}

// Results

pub struct PostResult {
    pub id: i64,
    pub title: String,
    pub content: Option<String>,
    pub user_id: i64,
    pub media: Option<String>,
    pub community_id: Option<i32>,
    pub created: DateTime<Utc>
}

// Responses

#[derive(Object)]
pub struct PostResponse {
    pub id: i64,
    pub title: String,
    pub content: Option<String>,
    pub user_id: i64,
    pub media: Vec<i64>,
    pub community_id: Option<i32>,
    pub created: DateTime<Utc>
}

#[derive(ApiResponse)]
enum GetPostResponse {
    #[oai(status = 200)]
    Ok(Json<post::PostResult>),
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

#[derive(ApiResponse)]
enum GetPostMediaResponse {
    #[oai(status = 200)]
    Ok(Attachment<Vec<u8>>),
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

#[derive(ApiResponse)]
enum GetUserPostsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<post::PostResult>>)
}

#[derive(ApiResponse)]
enum GetPostRecipesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<RecipeResult>>)
}

pub struct PostApi;

#[OpenApi(prefix_path = "/post", tag = "ApiTags::Post")]
impl PostApi {
    
    #[oai(path = "/", method = "post")]
    async fn create_post(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, post_payload: post::PostPayload, auth: JWTAuthorization) -> Result<()> {
        post::create_post(pool.0, storage.0, post_payload, auth.0).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_post(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetPostResponse> {
        let post = post::get_post(pool.0, id.0).await?;
        if let None = post {
            return Ok(GetPostResponse::NotFound(PlainText("Post not found".to_string())));
        }
        let post = post.unwrap();
        permission::user::is_following_or_public(pool.0, post.user_id, auth).await?;
        Ok(GetPostResponse::Ok(Json(post)))
    }

    #[oai(path = "/media/:media_id", method = "get")]
    async fn get_post_media(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, media_id: Path<i64>, auth: JWTAuthorization) -> Result<GetPostMediaResponse> {
        let post_media = post::get_post_media(pool.0, storage.0, media_id.0).await?;
        if let None = post_media {
            return Ok(GetPostMediaResponse::NotFound(PlainText("Media not found".to_string())));
        }
        let post_media = post_media.unwrap();
        permission::user::is_following_or_public(pool.0, post_media.user_id, auth).await?;
        Ok(GetPostMediaResponse::Ok(post_media.attachment))
    }

    #[oai(path = "/user/:user_id", method = "get")]
    async fn get_user_posts(&self, pool: Data<&MySqlPool>, user_id: Path<i64>, auth: JWTAuthorization) -> Result<GetUserPostsResponse> {
        permission::user::is_following_or_public(pool.0, user_id.0, auth).await?;
        let posts = post::get_user_posts(pool.0, user_id.0).await?;
        Ok(GetUserPostsResponse::Ok(Json(posts)))
    }

    #[oai(path = "/:id", method = "put")]
    async fn update_post(&self, pool: Data<&MySqlPool>, id: Path<i64>, update_post: Json<post::UpdatePost>, auth: JWTAuthorization) -> Result<()> {
        permission::post::owns_post(pool.0, id.0, auth).await?;
        post::update_post(pool.0, id.0, update_post.0).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_post(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::post::owns_post(pool.0, id.0, auth).await?;
        post::delete_post(pool.0, storage.0, id.0, auth.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/recipe", method = "get")]
    async fn get_post_recipes(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetPostRecipesResponse> {
        permission::post::is_visible(pool.0, id.0, auth).await?;
        let recipes: Vec<RecipeResult> = sqlx::query_as!(RecipeResult,
            "select recipe.id, recipe.title, recipe.description, recipe.ingredients, recipe.method, recipe.user_id, recipe.created
            from recipe
            inner join recipe_post on recipe.id = recipe_post.recipe_id
            where recipe_post.post_id = ?",
            id.0
            )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(GetPostRecipesResponse::Ok(Json(recipes)))
    }

    #[oai(path = "/:id/addrecipe/:recipe_id", method = "post")]
    async fn add_post_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, recipe_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::post::owns_post(pool.0, id.0, auth).await?;
        post::add_post_recipe(pool.0, id.0, recipe_id.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/removerecipe/:recipe_id", method = "delete")]
    async fn remove_post_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, recipe_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::post::owns_post(pool.0, id.0, auth).await?;
        post::remove_post_recipe(pool.0, id.0, recipe_id.0).await?;
        Ok(())
    }

}
