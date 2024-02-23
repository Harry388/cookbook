use poem_openapi::{OpenApi, payload::Json, param::Path, Tags, ApiResponse};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use futures::try_join;
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::model::{tag, recipe, post};
use crate::util::entry;

#[derive(Tags)]
enum ApiTags {
    Tag
}

// Responses

#[derive(ApiResponse)]
enum GetTagResponse {
    #[oai(status = 200)]
    Ok(Json<tag::TagResult>),
    #[oai(status = 404)]
    NotFound
}

#[derive(ApiResponse)]
enum GetTagsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<tag::TagResult>>),
    #[oai(status = 400)]
    BadRequest
}

#[derive(ApiResponse)]
enum UpdateTagsResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 400)]
    BadRequest
}

type GetTagEntriesResponse = Result<Json<Vec<entry::Entry>>>;

pub struct TagApi;

#[OpenApi(prefix_path = "/tag", tag = "ApiTags::Tag")]
impl TagApi {

    #[oai(path = "/:id", method = "get")]
    async fn get_tag(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetTagResponse> {
        let tag = tag::get_tag(pool.0, id.0, auth.0).await?;
        Ok(
            match tag {
                Some(t) => GetTagResponse::Ok(Json(t)),
                None => GetTagResponse::NotFound
            }
        )
    }

    #[oai(path = "/:id/entries", method = "get")]
    async fn get_tag_entries(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> GetTagEntriesResponse {
        let posts_fut = post::get_tag_posts(pool.0, id.0, auth.0);
        let recipes_fut = recipe::get_tag_recipes(pool.0, id.0, auth.0);
        let (posts, recipes) = try_join!(posts_fut, recipes_fut)?;
        let entries = entry::create_entries(posts, recipes);
        Ok(Json(entries))
    }

    #[oai(path = "/entry/:entry/:entry_id", method = "get")]
    async fn get_entry_tags(&self, pool: Data<&MySqlPool>, entry: Path<String>, entry_id: Path<i64>, auth: JWTAuthorization) -> Result<GetTagsResponse> {
        Ok(
            match entry.0.as_str() {
                "post" => {
                    permission::post::is_visible(pool.0, entry_id.0, auth).await?;
                    let tags = tag::get_post_tags(pool.0, entry_id.0, auth.0).await?;
                    GetTagsResponse::Ok(Json(tags))
                },
                "recipe" => {
                    permission::recipe::is_visible(pool.0, entry_id.0, auth).await?;
                    let tags = tag::get_recipe_tags(pool.0, entry_id.0, auth.0).await?;
                    GetTagsResponse::Ok(Json(tags))
                },
                _ => {
                    GetTagsResponse::BadRequest
                }
            }
        )
    }
    
    #[oai(path = "/entry/:entry/:entry_id", method = "post")]
    async fn add_entry_tags(&self, pool: Data<&MySqlPool>, entry: Path<String>, entry_id: Path<i64>, tags: Json<tag::Tags>, auth: JWTAuthorization) -> Result<UpdateTagsResponse> {
        Ok(
            match entry.0.as_str() {
                "post" => {
                    permission::post::owns_post(pool.0, entry_id.0, auth).await?;
                    let tag_ids = tag::create_tags(pool.0, tags.0).await?;
                    post::add_post_tags(pool.0, entry_id.0, tag_ids).await?;
                    UpdateTagsResponse::Ok
                },
                "recipe" => {
                    permission::recipe::owns_recipe(pool.0, entry_id.0, auth).await?;
                    let tag_ids = tag::create_tags(pool.0, tags.0).await?;
                    recipe::add_recipe_tags(pool.0, entry_id.0, tag_ids).await?;
                    UpdateTagsResponse::Ok
                },
                _ => {
                    UpdateTagsResponse::BadRequest
                }
            }
        )
    }

    #[oai(path = "/entry/:entry/:entry_id", method = "delete")]
    async fn remove_entry_tags(&self, pool: Data<&MySqlPool>, entry: Path<String>, entry_id: Path<i64>, tags: Json<Vec<i64>>, auth: JWTAuthorization) -> Result<UpdateTagsResponse> {
        Ok(
            match entry.0.as_str() {
                "post" => {
                    permission::post::owns_post(pool.0, entry_id.0, auth).await?;
                    post::remove_post_tags(pool.0, entry_id.0, tags.0).await?;
                    UpdateTagsResponse::Ok
                },
                "recipe" => {
                    permission::recipe::owns_recipe(pool.0, entry_id.0, auth).await?;
                    recipe::remove_recipe_tags(pool.0, entry_id.0, tags.0).await?;
                    UpdateTagsResponse::Ok
                },
                _ => {
                    UpdateTagsResponse::BadRequest
                }
            }
        )
    }

    #[oai(path = "/:id/follow", method = "post")]
    async fn follow_tag(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        tag::follow_tag(pool.0, id.0, auth.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/unfollow", method = "delete")]
    async fn unfollow_tag(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        tag::unfollow_tag(pool.0, id.0, auth.0).await?;
        Ok(())
    }
}
