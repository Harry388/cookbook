use poem_openapi::{OpenApi, payload::Json, Object, ApiResponse, Tags, param::Path};
use poem::{web::Data, error::InternalServerError, Result};
use sqlx::MySqlPool;
use crate::api::auth::JWTAuthorization;
use crate::api::post::{PostResult, PostResponse};

#[derive(Tags)]
enum ApiTags {
    Tag
}

// Inputs

#[derive(Object)]
struct Tag {
    tag: String
}

// Results

#[derive(Object)]
pub struct TagResult {
    pub id: i64,
    pub tag: String
}

// Responses

#[derive(ApiResponse)]
enum GetAllTagsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<TagResult>>)
}

#[derive(ApiResponse)]
enum GetTagPostsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<PostResponse>>)
}

pub struct TagApi;

#[OpenApi(prefix_path = "/tag", tag = "ApiTags::Tag")]
impl TagApi {

    #[oai(path = "/", method = "post")]
    async fn create_tag(&self, pool: Data<&MySqlPool>, tag: Json<Tag>, _auth: JWTAuthorization) -> Result<()> {
        sqlx::query!(
            "insert into tag (tag) values (?)",
            tag.tag
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/", method = "get")]
    async fn get_all_tags(&self, pool: Data<&MySqlPool>, _auth: JWTAuthorization) -> Result<GetAllTagsResponse> {
        let tags = sqlx::query_as!(TagResult,
            "select id, tag from tag",
            )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;
       Ok(GetAllTagsResponse::Ok(Json(tags))) 
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_tag(&self, pool: Data<&MySqlPool>, id: Path<i64>, _auth: JWTAuthorization) -> Result<()> {
        sqlx::query!(
            "delete from tag where id = ?",
            id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/:id/post", method = "get")]
    async fn get_tag_posts(&self, pool: Data<&MySqlPool>, id: Path<i64>, _auth: JWTAuthorization) -> Result<GetTagPostsResponse> {
        let posts: Vec<PostResult> = sqlx::query_as!(PostResult,
            "select post.id, post.title, post.content, post.user_id, group_concat(post_media.id) as media, post.created
            from post
            left join post_media on post.id = post_media.post_id
            inner join post_tag on post.id = post_tag.post_id
            where post_tag.tag_id = ?
            group by post.id",
            id.0
            )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;
        let mut post_response = vec![];
        for post in posts {
            let media = match post.media {
                Some(media_ids) => media_ids.split(",").map(|m| m.parse().unwrap()).collect(),
                None => vec![]
            };
            post_response.push(PostResponse { id: post.id, title: post.title, content: post.content, user_id: post.user_id, media, created: post.created });
        }
        Ok(GetTagPostsResponse::Ok(Json(post_response)))
    }

}
