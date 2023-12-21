use poem_openapi::{OpenApi, payload::{Json, PlainText}, Object, ApiResponse, Tags, types::Email, SecurityScheme, auth::ApiKey};
use poem::{web::Data, error::InternalServerError, Result, Request, http::StatusCode, error::Error};
use sqlx::MySqlPool;

use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

#[derive(Tags)]
enum ApiTags {
    Auth
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize
}

// Auth

#[derive(SecurityScheme, Clone, Copy)]
#[oai(
    ty = "api_key",
    key_in = "cookie",
    key_name = "token",
    checker = "token_checker"
)]
pub struct JWTAuthorization(pub i64);

async fn token_checker(req: &Request, req_key: ApiKey) -> Option<i64> {
    let jwt_secret = req.data::<String>().unwrap();
    let token_wrapped = 
        decode::<Claims>(&req_key.key, &DecodingKey::from_secret(jwt_secret.as_bytes()), &Validation::default());
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

#[derive(Object)]
struct LogIn {
    email: Email,
    password: String
}

// Results

struct LogInResult {
    id: i64,
    password: String
}

// Responses

#[derive(ApiResponse)]
enum LogInResponse {
    #[oai(status = 200)]
    Ok(#[oai(header = "set-cookie")] String),
    #[oai(status = 401)]
    InvalidLogIn(PlainText<String>)
}

#[derive(ApiResponse)]
enum LogOutResponse {
    #[oai(status = 200)]
    Ok(#[oai(header = "set-cookie")] String)
}

pub struct AuthApi;

#[OpenApi(prefix_path = "/auth", tag = "ApiTags::Auth")]
impl AuthApi {
    #[oai(path = "/login", method = "post")]
    async fn login(&self, pool: Data<&MySqlPool>, jwt_secret: Data<&String>, login: Json<LogIn>) -> Result<LogInResponse> {
        let user: Option<LogInResult> = sqlx::query_as!(LogInResult,
            "select id, password from user where email = ?",
            login.email.as_str()
            )
            .fetch_optional(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(
            match user {
                Some(user_data) => {
                    let verified = verify_password(&login.password, &user_data.password)?;
                    if !verified {
                        return Ok(LogInResponse::InvalidLogIn(PlainText("Invalid log in".to_string())))
                    }

                    let token = generate_token(user_data, jwt_secret.0)?;
                    LogInResponse::Ok(format!("token={}; HttpOnly; SameSite=strict; Path=/", token))
                },
                None => LogInResponse::InvalidLogIn(PlainText("Invalid log in".to_string()))
            }
        )
    }

    #[oai(path = "/logout", method = "post")]
    async fn logout(&self) -> LogOutResponse {
        LogOutResponse::Ok("token=deleted; Path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT".to_string())
    }

    #[oai(path = "/test", method = "get")]
    async fn test(&self, auth: JWTAuthorization) -> Json<i64> {
        Json(auth.0)
    }
}

fn create_server_error() -> Error {
    Error::from_string("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn generate_password_hash(password: &String) -> Result<String> {
    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_wrapped = argon2.hash_password(password, &salt);
    match password_wrapped {
        Ok(password_hash) => Ok(password_hash.to_string()),
        Err(_) => Err(create_server_error())
    }
}

fn verify_password(password: &String, hash: &String) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash);
    if let Err(_) = parsed_hash {
        return Err(create_server_error());
    }
    let parsed_hash = parsed_hash.unwrap();
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

fn generate_token(user: LogInResult, secret: &str) -> Result<String> {
    let user_string = serde_json::to_string(&user.id).unwrap();
    let claims = Claims {
        exp: 1732212702,
        sub: user_string
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(InternalServerError)?;
    Ok(token)
}