use poem_openapi::{OpenApi, payload::{Json, PlainText}, ApiResponse, param::Path, Tags};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::model::{post, community, user};

#[derive(Tags)]
enum ApiTags {
    Community
}

// Responses

#[derive(ApiResponse)]
enum GetCommunityResponse {
    #[oai(status = 200)]
    Ok(Json<community::CommunityResult>),
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

#[derive(ApiResponse)]
enum GetMembersResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<user::CommunityUserResult>>)
}

#[derive(ApiResponse)]
enum GetUserCommunitiesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<community::CommunityResult>>)
}

#[derive(ApiResponse)]
enum GetCommunityPostsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<post::PostResult>>)
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
    async fn create_community(&self, pool: Data<&MySqlPool>, community: Json<community::Community>, auth: JWTAuthorization) -> Result<()> {
        community::create_community(pool.0, community.0, auth.0).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_community(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetCommunityResponse> {
        let community = community::get_community(pool.0, id.0, auth.0).await?;
        if let None = community {
            return Ok(GetCommunityResponse::NotFound(PlainText("Community not found".to_string())));
        }
        let community = community.unwrap();
        Ok(GetCommunityResponse::Ok(Json(community)))
    }

    #[oai(path = "/:id/join", method = "post")]
    async fn join_community(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        let accepted = permission::community::is_public(pool.0, id.0).await.is_ok();
        community::join_community(pool.0, id.0, accepted, auth.0).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "put")]
    async fn update_community(&self, pool: Data<&MySqlPool>, id: Path<i64>, update: Json<community::UpdateCommunity>, auth: JWTAuthorization) -> Result<()> {
        permission::community::is_admin(pool.0, id.0, auth).await?;
        community::update_community(pool.0, id.0, update.0).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_community(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::community::is_admin(pool.0, id.0, auth).await?;
        community::delete_community(pool.0, id.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/members", method = "get")]
    async fn get_members(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetMembersResponse> {
        permission::community::is_in(pool.0, id.0, auth).await?;
        let members = user::get_community_users(pool.0, id.0).await?;
        Ok(GetMembersResponse::Ok(Json(members)))
    }

    #[oai(path = "/:id/requests", method = "get")]
    async fn get_requests(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetMembersResponse> {
        permission::community::is_admin(pool.0, id.0, auth).await?;
        let requests = user::get_community_requests(pool.0, id.0).await?;
        Ok(GetMembersResponse::Ok(Json(requests)))
    }

    #[oai(path = "/user/:user_id/", method = "get")]
    async fn get_user_communities(&self, pool: Data<&MySqlPool>, user_id: Path<i64>, auth: JWTAuthorization) -> Result<GetUserCommunitiesResponse> {
        permission::user::is_following_or_public(pool.0, user_id.0, auth).await?;
        let communities = community::get_user_communities(pool.0, user_id.0, auth.0).await?;
        Ok(GetUserCommunitiesResponse::Ok(Json(communities)))
    }

    #[oai(path = "/:id/post", method = "get")]
    async fn get_community_posts(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetCommunityPostsResponse> {
        permission::community::is_in(pool.0, id.0, auth).await?;
        let posts = post::get_community_posts(pool.0, id.0).await?;
        Ok(GetCommunityPostsResponse::Ok(Json(posts)))
    }

    #[oai(path = "/:id/user/:user_id", method = "put")]
    async fn update_community_user(&self, pool: Data<&MySqlPool>, id: Path<i64>, user_id: Path<i64>, update: Json<community::UpdateCommunityUser>, auth: JWTAuthorization) -> Result<UpdateCommunityUserResponse> {
        if user_id.0 == auth.0 {
            return Ok(UpdateCommunityUserResponse::Unauthorized)
        }
        permission::community::is_admin(pool.0, id.0, auth).await?;
        community::update_community_user(pool.0, id.0, user_id.0, update.0).await?;
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
            if community::has_one_admin(pool.0, id.0).await? {
                return Ok(LeaveCommunityResponse::OneAdmin(PlainText("User is only admin".to_string())))
            }
        }
        community::leave_community(pool.0, id.0, user_id.0).await?;
        Ok(LeaveCommunityResponse::Ok)
    }

    #[oai(path = "/:id/removepost/:post_id", method = "delete")]
    async fn remove_post(&self, pool: Data<&MySqlPool>, id: Path<i64>, post_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::community::is_admin(pool.0, id.0, auth).await?;
        post::remove_community(pool.0, post_id.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/acceptmember/:user_id", method = "put")]
    async fn accept_member(&self, pool: Data<&MySqlPool>, id: Path<i64>, user_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::community::is_admin(pool.0, id.0, auth).await?;
        community::accept_member(pool.0, id.0, user_id.0).await?;
        Ok(())
    }

}
