use poem_openapi::{OpenApi, payload::{Json, PlainText, Attachment, AttachmentType}, Object, ApiResponse, param::Path, Tags, Multipart, types::multipart::{Upload, JsonField}};
use poem::{web::Data, error::InternalServerError, Result};
use sqlx::{MySqlPool, types::chrono::{DateTime, Utc}};
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::storage::{Storage, dufs::DufsStorage};
use crate::api::post::{PostResult, PostResponse};

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

#[derive(Object)]
struct UpdateCommunityUser {
    permission: String 
}

// Results

#[derive(Object)]
struct CommunityResult {
    id: i64,
    title: String,
    description: Option<String>,
    created: DateTime<Utc>,
    users: i64,
    is_member: Option<f32>,
    is_admin: Option<f32>
}

#[derive(Object)]
struct GetUserResult {
    id: i64,
    username: String,
    display_name: String,
    permission: String
}

struct UserCountResult {
    count: i64
}

// Responses

#[derive(ApiResponse)]
enum GetCommunityResponse {
    #[oai(status = 200)]
    Ok(Json<CommunityResult>),
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

#[derive(ApiResponse)]
enum GetMembersResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<GetUserResult>>)
}

#[derive(ApiResponse)]
enum GetUserCommunitiesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<CommunityResult>>)
}

#[derive(ApiResponse)]
enum GetCommunityPostsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<PostResponse>>)
}

#[derive(ApiResponse)]
enum UpdateCommunityUserResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 401)]
    Unauthorized
}

#[derive(ApiResponse)]
enum LeaveCommunityResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 500)]
    OneAdmin(PlainText<String>),
    #[oai(status = 401)]
    Unauthorized
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
    async fn get_community(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetCommunityResponse> {
        let community = sqlx::query_as!(CommunityResult,
            "select id, title, description, created, count(*) as users,
            cast(sum(case when community_user.user_id = ? then 1 else 0 end) as float) as is_member,
            cast(sum(case when community_user.user_id = ? and community_user.permission = 'ADMIN' then 1 else 0 end) as float) as is_admin
            from community
            inner join community_user on community.id = community_user.community_id
            where community.id = ?
            group by community.id",
            auth.0, auth.0, id.0
            )
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?;
        if let None = community {
            return Ok(GetCommunityResponse::NotFound(PlainText("Community not found".to_string())));
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

    #[oai(path = "/:id/members", method = "get")]
    async fn get_members(&self, pool: Data<&MySqlPool>, id: Path<i64>, _auth: JWTAuthorization) -> Result<GetMembersResponse> {
        let members = sqlx::query_as!(GetUserResult,
             "select id, username, display_name, permission 
             from user inner join community_user on user.id = community_user.user_id
             where community_user.community_id = ?
             group by user.id, permission",
             id.0
             )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(GetMembersResponse::Ok(Json(members)))
    }

    #[oai(path = "/user/:user_id/", method = "get")]
    async fn get_user_communities(&self, pool: Data<&MySqlPool>, user_id: Path<i64>, auth: JWTAuthorization) -> Result<GetUserCommunitiesResponse> {
        permission::user::is_following_or_public(pool.0, user_id.0, auth).await?;
        let communities = sqlx::query_as!(CommunityResult,
            "with community_and_users as (
                select id, title, description, created, count(*) as users,
                cast(sum(case when community_user.user_id = ? then 1 else 0 end) as float) as is_member,
                cast(sum(case when community_user.user_id = ? and community_user.permission = 'ADMIN' then 1 else 0 end) as float) as is_admin
                from community
                inner join community_user on community.id = community_user.community_id
                group by community.id
            )
            select id, title, description, created, users, is_member, is_admin
            from community_and_users
            inner join community_user on community_user.community_id = community_and_users.id
            where community_user.user_id = ?",
            auth.0, auth.0, user_id.0
            )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(GetUserCommunitiesResponse::Ok(Json(communities)))
    }

    #[oai(path = "/:id/post", method = "get")]
    async fn get_community_posts(&self, pool: Data<&MySqlPool>, id: Path<i64>, _auth: JWTAuthorization) -> Result<GetCommunityPostsResponse> {
        let posts: Vec<PostResult> = sqlx::query_as!(PostResult,
            "select post.id, post.title, post.content, post.user_id, group_concat(post_media.id) as media, post.created, post.community_id
            from post
            left join post_media on post.id = post_media.post_id
            where post.community_id = ?
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
            post_response.push(PostResponse { id: post.id, title: post.title, content: post.content, user_id: post.user_id, media, created: post.created, community_id: post.community_id });
        }
        Ok(GetCommunityPostsResponse::Ok(Json(post_response)))
    }

    #[oai(path = "/:id/user/:user_id", method = "put")]
    async fn update_community_user(&self, pool: Data<&MySqlPool>, id: Path<i64>, user_id: Path<i64>, update: Json<UpdateCommunityUser>, auth: JWTAuthorization) -> Result<UpdateCommunityUserResponse> {
        if user_id.0 == auth.0 {
            return Ok(UpdateCommunityUserResponse::Unauthorized)
        }
        permission::community::is_admin(pool.0, id.0, auth).await?;
        sqlx::query!(
            "update community_user set permission = ?
            where community_id = ? and user_id = ?",
            update.permission, id.0, user_id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(UpdateCommunityUserResponse::Ok)
    }

    #[oai(path = "/:id/leave/:user_id", method = "delete")]
    async fn leave_community(&self, pool: Data<&MySqlPool>, id: Path<i64>, user_id: Path<i64>, auth: JWTAuthorization) -> Result<LeaveCommunityResponse> {
        let this_user_is_admin = permission::community::is_admin(pool.0, id.0, auth).await.is_ok();
        let this_user = permission::user::is_user(user_id.0, auth).is_ok();
        if !(this_user || this_user_is_admin) {
            return Ok(LeaveCommunityResponse::Unauthorized);
        }
        if this_user && this_user_is_admin {
            if self.has_one_admin(pool.0, id.0).await? {
                return Ok(LeaveCommunityResponse::OneAdmin(PlainText("User is only admin".to_string())))
            }
        }
        sqlx::query!(
            "delete from community_user where community_id = ? and user_id = ?",
            id.0, user_id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(LeaveCommunityResponse::Ok)
    }

    async fn has_one_admin(&self, pool: &MySqlPool, id: i64) -> Result<bool> {
        let admin_count = sqlx::query_as!(UserCountResult,
            "select count(*) as count
            from community
            inner join community_user on community.id = community_user.community_id
            where community.id = ? and community_user.permission = 'ADMIN'
            group by community.id",
            id
            )
            .fetch_optional(pool)
            .await
            .map_err(InternalServerError)?;
        Ok(
            match admin_count {
                Some(uc) => uc.count == 1,
                None => true
            }
        )
    }

}
