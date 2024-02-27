use poem_openapi::{OpenApi, payload::{Json, PlainText, Attachment}, ApiResponse, param::Path, Tags, types::multipart::JsonField, Multipart};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use futures::try_join;
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::storage::dufs::DufsStorage;
use crate::model::{post, recipe, comment, tag, like};

#[derive(Tags)]
enum ApiTags {
    Post
}

// Inputs

#[derive(Multipart)]
struct PostWithMediaWithTags {
    post: JsonField<post::Post>,
    media: post::Media,
    tags: tag::Tags
}

// Responses

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
    Ok(Json<Vec<recipe::RecipeResult>>)
}

#[derive(ApiResponse)]
enum GetPostCommentsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<comment::CommentResult>>)
}

pub struct PostApi;

#[OpenApi(prefix_path = "/post", tag = "ApiTags::Post")]
impl PostApi {
    
    #[oai(path = "/", method = "post")]
    async fn create_post(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, post: PostWithMediaWithTags, auth: JWTAuthorization) -> Result<()> {
        if let Some(community_id) = post.post.0.community_id {
            permission::community::is_in(pool.0, community_id, auth).await?;
        }
        let post_fut = post::create_post(pool.0, post.post.0, auth.0);
        let tags_fut = tag::create_tags(pool.0, post.tags);
        let (post_id, tag_ids) = try_join!(post_fut, tags_fut)?;
        let media_fut = post::add_post_post_media(pool.0, storage.0, post_id as i64, post.media, auth.0);
        let tags_fut = post::add_post_tags(pool.0, post_id as i64, tag_ids);
        try_join!(media_fut, tags_fut)?;
        Ok(())
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_post(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetPostResponse> {
        let post = post::get_post(pool.0, id.0, auth.0).await?;
        if let None = post {
            return Ok(GetPostResponse::NotFound(PlainText("Post not found".to_string())));
        }
        let post = post.unwrap();
        permission::post::is_visible(pool.0, id.0, auth).await?;
        Ok(GetPostResponse::Ok(Json(post)))
    }

    #[oai(path = "/media/:media_id", method = "get")]
    async fn get_post_media(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, media_id: Path<i64>, auth: JWTAuthorization) -> Result<GetPostMediaResponse> {
        let post_media = post::get_post_media(pool.0, storage.0, media_id.0).await?;
        if let None = post_media {
            return Ok(GetPostMediaResponse::NotFound(PlainText("Media not found".to_string())));
        }
        let post_media = post_media.unwrap();
        permission::post::is_visible(pool.0, post_media.post_id, auth).await?;
        Ok(GetPostMediaResponse::Ok(post_media.attachment))
    }

    #[oai(path = "/user/:user_id", method = "get")]
    async fn get_user_posts(&self, pool: Data<&MySqlPool>, user_id: Path<i64>, auth: JWTAuthorization) -> Result<GetUserPostsResponse> {
        permission::user::is_following_or_public(pool.0, user_id.0, auth).await?;
        let posts = post::get_user_posts(pool.0, user_id.0, auth.0).await?;
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
        let recipes = recipe::get_post_recipes(pool.0, id.0, auth.0).await?;
        Ok(GetPostRecipesResponse::Ok(Json(recipes)))
    }

    #[oai(path = "/:id/addrecipe/:recipe_id", method = "post")]
    async fn add_post_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, recipe_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::post::owns_post(pool.0, id.0, auth).await?;
        permission::recipe::is_visible(pool.0, recipe_id.0, auth).await?;
        post::add_post_recipe(pool.0, id.0, recipe_id.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/removerecipe/:recipe_id", method = "delete")]
    async fn remove_post_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, recipe_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::post::owns_post(pool.0, id.0, auth).await?;
        post::remove_post_recipe(pool.0, id.0, recipe_id.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/comment", method = "get")]
    async fn get_post_comments(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetPostCommentsResponse> {
        permission::post::is_visible(pool.0, id.0, auth).await?;
        let comments = comment::get_post_comments(pool.0, id.0).await?;
        Ok(GetPostCommentsResponse::Ok(Json(comments)))
    }

    #[oai(path = "/:id/comment", method = "post")]
    async fn create_post_comment(&self, pool: Data<&MySqlPool>, id: Path<i64>, comment: Json<comment::Comment>, auth: JWTAuthorization) -> Result<()> {
        permission::post::is_visible(pool.0, id.0, auth).await?;
        comment::create_post_comment(pool.0, comment.0, id.0, auth.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/like", method = "post")]
    async fn like_post(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::post::is_visible(pool.0, id.0, auth).await?;
        like::like_post(pool.0, id.0, auth.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/like", method = "delete")]
    async fn unlike_post(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::post::is_visible(pool.0, id.0, auth).await?;
        like::unlike_post(pool.0, id.0, auth.0).await?;
        Ok(())
    }

}
