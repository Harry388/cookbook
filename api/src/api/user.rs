use poem_openapi::{OpenApi, payload::{Json, PlainText, Attachment}, ApiResponse, param::Path, Tags, Object};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use futures::try_join;
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::storage::dufs::DufsStorage;
use crate::model::{user, post, recipe};
use crate::util::entry;

#[derive(Tags)]
enum ApiTags {
    User
}

// Responses

#[derive(ApiResponse)]
enum GetUserResponse {
    #[oai(status = 200)]
    Ok(Json<user::UserResult>),
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

#[derive(ApiResponse)]
enum FollowResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<user::FollowResult>>)
}

#[derive(ApiResponse)]
enum GetProfilePicResponse {
    #[oai(status = 200)]
    Ok(Attachment<Vec<u8>>),
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

#[derive(Object)]
struct FeedEntries {
    posts: Vec<post::PostResult>,
    recipes: Vec<recipe::RecipeResult>
}

type GetFeedResponse = Json<Vec<entry::Entry>>;

pub struct UserApi;

#[OpenApi(prefix_path = "/user", tag = "ApiTags::User")]
impl UserApi {
    #[oai(path = "/", method = "post")]
    async fn create_user(&self, pool: Data<&MySqlPool>, user: Json<user::User>) -> Result<Json<u64>> {
        let id = user::create_user(pool.0, user.0).await?;
        Ok(Json(id))
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_user(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetUserResponse> {
        let user = user::get_user(pool.0, id.0, auth.0).await?;
        Ok(
            match user {
                Some(user_data) => GetUserResponse::Ok(Json(user_data)),
                None => GetUserResponse::NotFound(PlainText("User not found".to_string()))
            }
        )
    }

    #[oai(path = "/:id", method = "put")]
    async fn update_user(&self, pool: Data<&MySqlPool>, id: Path<i64>, user: Json<user::UpdateUser>, auth: JWTAuthorization) -> Result<()> {
        permission::user::is_user(id.0, auth)?;
        user::update_user(pool.0, id.0, user.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/pfp", method = "put")]
    async fn set_profile_pic(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, id: Path<i64>, pfp: user::SetProfilePic, auth: JWTAuthorization) -> Result<()> {
        permission::user::is_user(id.0, auth)?;
        user::set_profile_pic(pool.0, storage.0, id.0, pfp).await?;
        Ok(())
    }

    #[oai(path = "/:id/pfp", method = "get")]
    async fn get_profile_pic(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, id: Path<i64>, _auth: JWTAuthorization) -> Result<GetProfilePicResponse> {
        let result = user::get_profile_pic(pool.0, storage.0, id.0).await?;
        Ok(
            match result {
                user::ProfilePicResult::PicNotFound => {
                    GetProfilePicResponse::NotFound(PlainText("Profile pic not found".to_string()))
                },
                user::ProfilePicResult::UserNotFound => {
                    GetProfilePicResponse::NotFound(PlainText("User not found".to_string()))
                },
                user::ProfilePicResult::Ok(attachment) => {
                    GetProfilePicResponse::Ok(attachment)
                }
            }
        )
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_user(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::user::is_user(id.0, auth)?;
        user::delete_user(pool.0, storage.0, id.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/follow/:follow_id", method = "post")]
    async fn follow(&self, pool: Data<&MySqlPool>, id: Path<i64>, follow_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::user::is_user(id.0, auth)?;
        user::follow(pool.0, id.0, follow_id.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/unfollow/:follow_id", method = "delete")]
    async fn unfollow(&self, pool: Data<&MySqlPool>, id: Path<i64>, follow_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::user::in_user_list(&vec![id.0, follow_id.0], auth)?;
        user::unfollow(pool.0, id.0, follow_id.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/followers", method = "get")]
    async fn get_followers(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<FollowResponse> {
        permission::user::is_following_or_public(pool.0, id.0, auth).await?;
        let followers = user::get_followers(pool.0, id.0).await?;
        Ok(FollowResponse::Ok(Json(followers)))
    }

    #[oai(path = "/:id/following", method = "get")]
    async fn get_following(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<FollowResponse> {
        permission::user::is_following_or_public(pool.0, id.0, auth).await?;
        let following = user::get_following(pool.0, id.0).await?;
        Ok(FollowResponse::Ok(Json(following)))
    }

    #[oai(path = "/feed", method = "get")]
    async fn get_feed(&self, pool: Data<&MySqlPool>, auth: JWTAuthorization) -> Result<GetFeedResponse> {
        let posts_fut = post::get_feed_posts(pool.0, auth.0);
        let recipes_fut = recipe::get_feed_recipes(pool.0, auth.0);
        let (posts, recipes) = try_join!(posts_fut, recipes_fut)?;
        let entries = entry::create_entries(posts, recipes);
        Ok(Json(entries))
    }
}
