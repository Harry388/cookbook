use poem_openapi::{OpenApi, payload::{Json, PlainText}, Object, ApiResponse, param::Path, Tags, types::Email};
use poem::{web::Data, error::InternalServerError, Result};
use sqlx::MySqlPool;
use crate::api::auth::{generate_password_hash, JWTAuthorization};
use crate::permission;

#[derive(Tags)]
enum ApiTags {
    User
}

// Inputs

#[derive(Object)]
struct User {
    username: String,
    display_name: String,
    email: Email,
    password: String,
    bio: Option<String>,
    pfp: Option<String>
}

#[derive(Object)]
struct UpdateUser {
    display_name: Option<String>,
    bio: Option<String>,
    pfp: Option<String>
}

// Results

#[derive(Object)]
struct GetUserResult {
    id: i64,
    username: String,
    display_name: String,
    bio: Option<String>,
    pfp: Option<String>,
    public: i8,
    following: i64,
    followers: i64,
    is_following: Option<f32>
}

#[derive(Object)]
struct FollowResult {
    id: i64,
    username: String,
    display_name: String,
    pfp: Option<String>
}

// Responses

#[derive(ApiResponse)]
enum GetUserResponse {
    #[oai(status = 200)]
    Ok(Json<GetUserResult>),
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

#[derive(ApiResponse)]
enum FollowResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<FollowResult>>)
}

pub struct UserApi;

#[OpenApi(prefix_path = "/user", tag = "ApiTags::User")]
impl UserApi {
    #[oai(path = "/", method = "post")]
    async fn create_user(&self, pool: Data<&MySqlPool>, user: Json<User>) -> Result<Json<u64>> {
        let password = generate_password_hash(&user.password)?;
        let id = sqlx::query!( 
            "insert into user (username, display_name, email, password, bio, pfp, public)
            values (?,?,?,?,?,?,?)",
            user.username, user.display_name, user.email.0, password, user.bio, user.pfp, true
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?
            .last_insert_id();
        Ok(Json(id))
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_user(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetUserResponse> {
        let user = sqlx::query_as!(GetUserResult,
            "select id, username, display_name, bio, pfp, public, 
            count(following.user_id) as following, count(followers.following_id) as followers,
            cast(sum(case when followers.user_id = ? then 1 else 0 end) as float) as is_following
            from user 
            left join following as following on user.id = following.user_id 
            left join following as followers on user.id = followers.following_id
            where id = ?
            group by id",
            auth.0, id.0
            )
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(
            match user {
                Some(user_data) => GetUserResponse::Ok(Json(user_data)),
                None => GetUserResponse::NotFound(PlainText("User not found".to_string()))
            }
        )
    }

    #[oai(path = "/:id", method = "put")]
    async fn update_user(&self, pool: Data<&MySqlPool>, id: Path<i64>, user: Json<UpdateUser>, auth: JWTAuthorization) -> Result<()> {
        permission::user::is_user(id.0, auth)?;
        sqlx::query!(
            "update user set display_name = coalesce(?, display_name), bio = coalesce(?, bio), pfp = coalesce(?, pfp) where id = ?",
            user.display_name, user.bio, user.pfp, id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_user(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::user::is_user(id.0, auth)?;
        sqlx::query!(
            "delete from user where id = ?",
            id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/:id/follow/:follow_id", method = "post")]
    async fn follow(&self, pool: Data<&MySqlPool>, id: Path<i64>, follow_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::user::is_user(id.0, auth)?;
        sqlx::query!(
            "insert into following (user_id, following_id) values (?, ?)",
            id.0, follow_id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/:id/unfollow/:follow_id", method = "delete")]
    async fn unfollow(&self, pool: Data<&MySqlPool>, id: Path<i64>, follow_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::user::in_user_list(&vec![id.0, follow_id.0], auth)?;
        sqlx::query!(
            "delete from following where user_id = ? and following_id = ?",
            id.0, follow_id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }

    #[oai(path = "/:id/followers", method = "get")]
    async fn get_followers(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<FollowResponse> {
        permission::user::is_following_or_public(pool.0, id.0, auth).await?;
        let followers = sqlx::query_as!(FollowResult,
            "select id, username, display_name, pfp from user where id in (
                select user_id from following where following_id = ?
            )",
            id.0
            )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(FollowResponse::Ok(Json(followers)))
    }

    #[oai(path = "/:id/following", method = "get")]
    async fn get_following(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<FollowResponse> {
        permission::user::is_following_or_public(pool.0, id.0, auth).await?;
        let followers = sqlx::query_as!(FollowResult,
            "select id, username, display_name, pfp from user where id in (
                select following_id from following where user_id = ?
            )",
            id.0
            )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(FollowResponse::Ok(Json(followers)))
    }
}