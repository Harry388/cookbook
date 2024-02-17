use poem_openapi::{OpenApi, payload::Json, param::Path, Tags, Object};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use futures::try_join;
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::model::{tag, recipe, post};

#[derive(Tags)]
enum ApiTags {
    Tag
}

// Responses

type GetTagsResponse = Result<Json<Vec<tag::TagResult>>>;

#[derive(Object)]
struct TagEntries {
    posts: Vec<post::PostResult>,
    recipes: Vec<recipe::RecipeResult>
}

type GetTagEntriesResponse = Result<Json<TagEntries>>;


pub struct TagApi;

#[OpenApi(prefix_path = "/tag", tag = "ApiTags::Tag")]
impl TagApi {

    #[oai(path = "/recipe/:recipe_id", method = "get")]
    async fn get_recipe_tags(&self, pool: Data<&MySqlPool>, recipe_id: Path<i64>, auth: JWTAuthorization) -> GetTagsResponse {
        permission::recipe::is_visible(pool.0, recipe_id.0, auth).await?;
        let tags = tag::get_recipe_tags(pool.0, recipe_id.0).await?;
        Ok(Json(tags))
    }

    #[oai(path = "/post/:post_id", method = "get")]
    async fn get_post_tags(&self, pool: Data<&MySqlPool>, post_id: Path<i64>, auth: JWTAuthorization) -> GetTagsResponse {
        permission::post::is_visible(pool.0, post_id.0, auth).await?;
        let tags = tag::get_post_tags(pool.0, post_id.0).await?;
        Ok(Json(tags))
    }

    #[oai(path = "/:id/entries", method = "get")]
    async fn get_tag_entries(&self, pool: Data<&MySqlPool>, id: Path<i64>, _auth: JWTAuthorization) -> GetTagEntriesResponse {
        let posts_fut = post::get_tag_posts(pool.0, id.0);
        let recipes_fut = recipe::get_tag_recipes(pool.0, id.0);
        let (posts, recipes) = try_join!(posts_fut, recipes_fut)?;
        let entries = TagEntries { posts, recipes };
        Ok(Json(entries))
    }
    
}
