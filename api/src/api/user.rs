use poem_openapi::{OpenApi, payload::{Json, PlainText}, Object, ApiResponse, param::Path};
use poem::web::Data;
use sqlx::MySqlPool;

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

#[derive(ApiResponse)]
enum CreateUserResponse {
    #[oai(status = 200)]
    Ok(Json<u64>)
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct FindUser {
    id: i64,
    username: String,
    display_name: String,
    email: String,
    bio: Option<String>,
    pfp: Option<String>
}

#[derive(ApiResponse)]
enum FindUserResponse {
    #[oai(status = 200)]
    Ok(Json<FindUser>),
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

pub struct UserApi;

#[OpenApi(prefix_path = "/user")]
impl UserApi {
    #[oai(path = "/", method = "post")]
    async fn create_user(&self, pool: Data<&MySqlPool>, user: Json<User>) -> CreateUserResponse {
        let user_data = user.0;
        let id = sqlx::query_as!(u64, 
            "insert into user (username, display_name, email, password, bio, pfp)
            values (?,?,?,?,?,?)",
            user_data.username, user_data.display_name, user_data.email, user_data.password, user_data.bio, user_data.pfp
            )
            .execute(pool.0)
            .await
            .unwrap()
            .last_insert_id();
        CreateUserResponse::Ok(Json(id))
    }

    #[oai(path = "/:id", method = "get")]
    async fn find_user(&self, pool: Data<&MySqlPool>, id: Path<i64>) -> FindUserResponse {
        let user = sqlx::query_as!(FindUser,
            "select id, username, display_name, email, bio, pfp from user where id = ?", id.0
            )
            .fetch_optional(pool.0)
            .await
            .unwrap();
        match user {
            Some(user_data) => FindUserResponse::Ok(Json(user_data)),
            None => FindUserResponse::NotFound(PlainText("User not found".to_string()))
        }
    }
}