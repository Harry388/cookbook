use poem_openapi::{OpenApi, payload::{Json, PlainText}, Object, ApiResponse, Tags, types::Email, SecurityScheme, auth::Bearer};
use poem::{web::Data, error::InternalServerError, Result, Request};
use sqlx::MySqlPool;

use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

use crate::api::user::FindUserResult;

#[derive(Tags)]
enum ApiTags {
    Auth
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize
}

// Auth

#[derive(SecurityScheme)]
#[oai(
    ty = "bearer",
    checker = "token_checker"
)]
struct JWTAuthorization(i64);

async fn token_checker(_req: &Request, bearer: Bearer) -> Option<i64> {
    let token_wrapped = decode::<Claims>(&bearer.token, &DecodingKey::from_secret("secret".as_ref()), 
    &Validation::default());
    match token_wrapped {
        Ok(token) => {
            let user_wrapped = serde_json::from_str(&token.claims.sub);
            match user_wrapped {
                Ok(user) => Some(user),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

// Inputs

#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct LogIn {
    email: Email,
    password: String
}

// Responses

#[derive(ApiResponse)]
enum LogInResponse {
    #[oai(status = 200)]
    Ok(PlainText<String>),
    #[oai(status = 401)]
    InvalidLogIn(PlainText<String>)
}

pub struct AuthApi;

#[OpenApi(prefix_path = "/auth", tag = "ApiTags::Auth")]
impl AuthApi {
    #[oai(path = "/login", method = "post")]
    async fn login(&self, pool: Data<&MySqlPool>, login: Json<LogIn>) -> Result<LogInResponse> {
        let user: Option<FindUserResult> = sqlx::query_as!(FindUserResult,
            "select id, username, display_name, email, bio, pfp from user where email = ? and password = ?",
            login.email.as_str(), login.password
            )
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(
            match user {
                Some(user_data) => {
                    let user_string = serde_json::to_string(&user_data.id).unwrap();
                    let claims = Claims {
                        exp: 1700507077,
                        sub: user_string
                    };
                    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
                        .map_err(InternalServerError)?;
                    LogInResponse::Ok(PlainText(token))
                },
                None => LogInResponse::InvalidLogIn(PlainText("Invalid log in".to_string()))
            }
        )
    }

    #[oai(path = "/test", method = "get")]
    async fn hello(&self, auth: JWTAuthorization) -> Json<i64> {
        Json(auth.0)
    }
}