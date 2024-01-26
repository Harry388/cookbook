use poem_openapi::{OpenApi, payload::{Json, PlainText, Attachment, AttachmentType}, Object, ApiResponse, param::Path, Tags, Multipart, types::multipart::{Upload, JsonField}};
use poem::{web::Data, error::InternalServerError, Result};
use sqlx::{MySqlPool, types::chrono::{DateTime, Utc}};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::storage::{Storage, dufs::DufsStorage};
use crate::api::recipe::RecipeResult;
use crate::api::tag::TagResult;

#[derive(Tags)]
enum ApiTags {
    Post
}

// Inputs

#[derive(Object)]
struct Post {
    title: Option<String>,
    content: Option<String>
}

#[derive(Multipart)]
struct PostPayload {
    post: JsonField<Post>,
    media: Vec<Upload>
}

#[derive(Object)]
struct UpdatePost {
    title: Option<String>,
    content: Option<String>
}


// Results

pub struct PostResult {
    pub id: i64,
    pub title: String,
    pub content: Option<String>,
    pub user_id: i64,
    pub media: Option<String>,
    pub created: DateTime<Utc>
}

struct PostMediaResult {
    user_id: i64,
    uri: String
}

// Responses

#[derive(Object)]
pub struct PostResponse {
    pub id: i64,
    pub title: String,
    pub content: Option<String>,
    pub user_id: i64,
    pub media: Vec<i64>,
    pub created: DateTime<Utc>
}

#[derive(ApiResponse)]
enum GetPostResponse {
    #[oai(status = 200)]
    Ok(Json<PostResponse>),
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
    Ok(Json<Vec<PostResponse>>)
}

#[derive(ApiResponse)]
enum GetPostRecipesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<RecipeResult>>)
}

#[derive(ApiResponse)]
enum GetPostTagsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<TagResult>>)
}

pub struct PostApi;

#[OpenApi(prefix_path = "/post", tag = "ApiTags::Post")]
impl PostApi {
    
    #[oai(path = "/", method = "post")]
    async fn create_post(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, post_payload: PostPayload, auth: JWTAuthorization) -> Result<()> {
        let post = post_payload.post.0;
        let post_id = sqlx::query!( 
            "insert into post (title, content, user_id)
            values (?,?,?)",
            post.title, post.content, auth.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?
            .last_insert_id();
        for media in post_payload.media {
            let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string();
            let path = format!("user/{}/post/{}/{}", auth.0, post_id, time);
            let media_path = storage.0.put_file(&path, media).await?;
            sqlx::query!( 
                "insert into post_media (uri, post_id)
                values (?,?)",
                media_path, post_id
                )
                .execute(pool.0)
                .await
                .map_err(InternalServerError)?;
        }
        Ok(())
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_post(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetPostResponse> {
        let post = sqlx::query_as!(PostResult,
            "select post.id, post.title, post.content, post.user_id, group_concat(post_media.id) as media, created
            from post left join post_media on post.id = post_media.post_id
            where post.id = ?
            group by post.id",
            id.0
            )
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?;
        if let None = post {
            return Ok(GetPostResponse::NotFound(PlainText("Post not found".to_string())));
        }
        let post: PostResult = post.unwrap();
        permission::user::is_following_or_public(pool.0, post.user_id, auth).await?;
        let media = match post.media {
            Some(media_ids) => media_ids.split(",").map(|m| m.parse().unwrap()).collect(),
            None => vec![]
        };
        let post = PostResponse { id: post.id, title: post.title, content: post.content, user_id: post.user_id, media, created: post.created };
        Ok(GetPostResponse::Ok(Json(post)))
    }

    #[oai(path = "/media/:media_id", method = "get")]
    async fn get_post_media(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, media_id: Path<i64>, auth: JWTAuthorization) -> Result<GetPostMediaResponse> {
        let post_media = sqlx::query_as!(PostMediaResult,
            "select user_id, uri from post_media inner join post on post.id = post_media.post_id where post_media.id = ?",
            media_id.0
            )
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?;
        if let None = post_media {
            return Ok(GetPostMediaResponse::NotFound(PlainText("Post not found".to_string())));
        }
        let post_media: PostMediaResult = post_media.unwrap();
        permission::user::is_following_or_public(pool.0, post_media.user_id, auth).await?;
        let media = storage.0.get_file(&post_media.uri).await?;
        let attachment = 
            Attachment::new(media)
            .attachment_type(AttachmentType::Inline)
            .filename(post_media.uri);
        Ok(GetPostMediaResponse::Ok(attachment))
    }

    #[oai(path = "/user/:user_id", method = "get")]
    async fn get_user_posts(&self, pool: Data<&MySqlPool>, user_id: Path<i64>, auth: JWTAuthorization) -> Result<GetUserPostsResponse> {
        permission::user::is_following_or_public(pool.0, user_id.0, auth).await?;
        let posts: Vec<PostResult> = sqlx::query_as!(PostResult,
            "select post.id, post.title, post.content, post.user_id, group_concat(post_media.id) as media, created
            from post left join post_media on post.id = post_media.post_id
            where post.user_id = ?
            group by post.id",
            user_id.0
            )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;
        let mut full_posts = vec![];
        for post in posts {
            let media = match post.media {
                Some(media_ids) => media_ids.split(",").map(|m| m.parse().unwrap()).collect(),
                None => vec![]
            };
            full_posts.push(PostResponse { id: post.id, title: post.title, content: post.content, user_id: post.user_id, media, created: post.created })
        }
        Ok(GetUserPostsResponse::Ok(Json(full_posts)))
    }

    #[oai(path = "/:id", method = "put")]
    async fn update_post(&self, pool: Data<&MySqlPool>, id: Path<i64>, update_post: Json<UpdatePost>, auth: JWTAuthorization) -> Result<()> {
        permission::post::owns_post(pool.0, id.0, auth).await?;
        sqlx::query!(
            "update post set title = coalesce(?, title), content = coalesce(?, content) where id = ?",
            update_post.title, update_post.content, id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_post(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::post::owns_post(pool.0, id.0, auth).await?;
        sqlx::query!(
            "delete from post where id = ?",
            id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        storage.delete_file(&format!("user/{}/post/{}", auth.0, id.0)).await?;
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
        sqlx::query!(
            "insert into recipe_post (recipe_id, post_id) values (?,?)",
            recipe_id.0, id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/:id/removerecipe/:recipe_id", method = "delete")]
    async fn remove_post_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, recipe_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::post::owns_post(pool.0, id.0, auth).await?;
        sqlx::query!(
            "delete from recipe_post where recipe_id = ? and post_id = ?",
            recipe_id.0, id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/:id/tag", method = "get")]
    async fn get_post_tags(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetPostTagsResponse> {
        permission::post::is_visible(pool.0, id.0, auth).await?;
        let tags: Vec<TagResult> = sqlx::query_as!(TagResult,
            "select id, tag
            from tag
            inner join post_tag on tag.id = post_tag.tag_id
            where post_tag.post_id = ?",
            id.0
            )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(GetPostTagsResponse::Ok(Json(tags)))
    }

    #[oai(path = "/:id/addtag/:tag_id", method = "post")]
    async fn add_post_tag(&self, pool: Data<&MySqlPool>, id: Path<i64>, tag_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::post::owns_post(pool.0, id.0, auth).await?;
        sqlx::query!(
            "insert into post_tag (tag_id, post_id) values (?,?)",
            tag_id.0, id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/:id/removetag/:tag_id", method = "delete")]
    async fn remove_post_tag(&self, pool: Data<&MySqlPool>, id: Path<i64>, tag_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::post::owns_post(pool.0, id.0, auth).await?;
        sqlx::query!(
            "delete from post_tag where tag_id = ? and post_id = ?",
            tag_id.0, id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

}
