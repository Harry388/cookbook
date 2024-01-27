use poem_openapi::{OpenApi, payload::{Json, PlainText, Attachment, AttachmentType}, Object, ApiResponse, param::Path, Tags, Multipart, types::multipart::{Upload, JsonField}};
use poem::{web::Data, error::InternalServerError, Result};
use sqlx::{MySqlPool, types::chrono::{DateTime, Utc}};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::storage::{Storage, dufs::DufsStorage};
use crate::api::recipe::RecipeResult;

#[derive(Tags)]
enum ApiTags {
    Community
}

// Inputs

#[derive(Object)]
struct Community {
    title: String,
    description: Option<String>
}

#[derive(Object)]
struct UpdateCommunity {
    title: Option<String>,
    description: Option<String>
}

// Results

#[derive(Object)]
struct CommunityResult {
    id: i64,
    title: String,
    description: Option<String>,
    pfp: Option<String>,
    created: DateTime<Utc>,
    users: i64
}

// Responses

#[derive(ApiResponse)]
enum GetCommunityResponse {
    #[oai(status = 200)]
    Ok(Json<CommunityResult>),
    #[oai(status = 404)]
    NotFound
}

pub struct CommunityApi;

#[OpenApi(prefix_path = "community", tag = "ApiTags::Community")]
impl CommunityApi {

    #[oai(path = "/", method = "post")]
    async fn create_community(&self, pool: Data<&MySqlPool>, community: Json<Community>, auth: JWTAuthorization) -> Result<()> {
        let community_id = sqlx::query!(
            "insert into community (title, description) values (?, ?)",
            community.title, community.description
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?
            .last_insert_id();
        sqlx::query!(
            "insert into community_user (community_id, user_id, permission)
            values (?, ?, 'ADMIN')",
            community_id, auth.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_community(&self, pool: Data<&MySqlPool>, id: Path<i64>, _auth: JWTAuthorization) -> Result<GetCommunityResponse> {
        let community = sqlx::query_as!(CommunityResult,
            "select id, title, description, pfp, created, count(*) as users
            from community
            inner join community_user on community.id = community_user.community_id
            where community.id = ?
            group by community.id",
            id.0
            )
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?;
        if let None = community {
            return Ok(GetCommunityResponse::NotFound);
        }
        let community = community.unwrap();
        Ok(GetCommunityResponse::Ok(Json(community)))
    }

    #[oai(path = "/:id/join", method = "post")]
    async fn join_community(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        sqlx::query!(
            "insert into community_user (community_id, user_id, permission) 
            values (?, ?, 'USER')",
            id.0, auth.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/:id", method = "put")]
    async fn update_community(&self, pool: Data<&MySqlPool>, id: Path<i64>, update: Json<UpdateCommunity>, auth: JWTAuthorization) -> Result<()> {
        permission::community::is_admin(pool.0, id.0, auth).await?;
        sqlx::query!(
            "update community set title = coalesce(?, title), description = coalesce(?, description) where id = ?",
            update.title, update.description, id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_community(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::community::is_admin(pool.0, id.0, auth).await?;
        sqlx::query!(
            "delete from community where id = ?",
            id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }
        
}
