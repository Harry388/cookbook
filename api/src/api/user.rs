use poem_openapi::{OpenApi, payload::{Json, PlainText}, Object, ApiResponse, param::Path, Tags};
use poem::{web::Data, error::InternalServerError, Result};
use sqlx::MySqlPool;

#[derive(Tags)]
enum ApiTags {
    User,
}

// Inputs

#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct User {
    #[oai(read_only)]
    id: i64,
    username: String,
    display_name: String,
    email: String,
    password: String,
    bio: Option<String>,
    pfp: Option<String>
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct UpdateUser {
    display_name: Option<String>,
    bio: Option<String>,
    pfp: Option<String>
}

// Results

#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct FindUserResult {
    id: i64,
    username: String,
    display_name: String,
    email: String,
    bio: Option<String>,
    pfp: Option<String>
}

// Responses

#[derive(ApiResponse)]
enum FindUserResponse {
    #[oai(status = 200)]
    Ok(Json<FindUserResult>),
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

pub struct UserApi;

#[OpenApi(prefix_path = "/user", tag = "ApiTags::User")]
impl UserApi {
    #[oai(path = "/", method = "post")]
    async fn create_user(&self, pool: Data<&MySqlPool>, user: Json<User>) -> Result<Json<u64>> {
        let id = sqlx::query_as!(u64, 
            "insert into user (username, display_name, email, password, bio, pfp)
            values (?,?,?,?,?,?)",
            user.username, user.display_name, user.email, user.password, user.bio, user.pfp
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?
            .last_insert_id();
        Ok(Json(id))
    }

    #[oai(path = "/:id", method = "get")]
    async fn find_user(&self, pool: Data<&MySqlPool>, id: Path<i64>) -> Result<FindUserResponse> {
        let user = sqlx::query_as!(FindUserResult,
            "select id, username, display_name, email, bio, pfp from user where id = ?", id.0
            )
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?;
        match user {
            Some(user_data) => Ok(FindUserResponse::Ok(Json(user_data))),
            None => Ok(FindUserResponse::NotFound(PlainText("User not found".to_string())))
        }
    }

    #[oai(path = "/:id", method = "put")]
    async fn update_user(&self, pool: Data<&MySqlPool>, id: Path<i64>, user: Json<UpdateUser>) -> Result<FindUserResponse> {
        sqlx::query!(
            "update user set display_name = coalesce(?, display_name), bio = coalesce(?, bio), pfp = coalesce(?, pfp) where id = ?",
            user.display_name, user.bio, user.pfp, id.0
            )
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(self.find_user(pool, id).await?)
    }
}