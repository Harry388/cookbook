use poem_openapi::{OpenApi, payload::{Json, PlainText}, Object, ApiResponse, Tags, types::Email, SecurityScheme, auth::ApiKey};
use poem::{web::Data, error::InternalServerError, Result, Request, http::StatusCode};
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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize
}

// Auth

#[derive(SecurityScheme)]
#[oai(
    ty = "api_key",
    key_in = "cookie",
    key_name = "token",
    checker = "token_checker"
)]
struct JWTAuthorization(i64);

async fn token_checker(_req: &Request, req_key: ApiKey) -> Option<i64> {
    let token_wrapped = decode::<Claims>(&req_key.key, &DecodingKey::from_secret("secret".as_ref()), &Validation::default());
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

// Results

#[derive(Debug, Object, Clone, Eq, PartialEq)]
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
    async fn login(&self, pool: Data<&MySqlPool>, login: Json<LogIn>) -> Result<LogInResponse> {
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

                    let token = generate_token(user_data)?;
                    LogInResponse::Ok(format!("token={}; HttpOnly; SameSite=strict", token))
                },
                None => LogInResponse::InvalidLogIn(PlainText("Invalid log in".to_string()))
            }
        )
    }

    #[oai(path = "/logout", method = "post")]
    async fn logout(&self, _auth: JWTAuthorization) -> LogOutResponse {
        LogOutResponse::Ok("token=deleted; expires=Thu, 01 Jan 1970 00:00:00 GMT".to_string())
    }

    #[oai(path = "/test", method = "get")]
    async fn hello(&self, auth: JWTAuthorization) -> Json<i64> {
        Json(auth.0)
    }
}

fn create_server_error() -> poem::error::Error {
    poem::error::Error::from_string("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
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

fn generate_token(user: LogInResult) -> Result<String> {
    let user_string = serde_json::to_string(&user.id).unwrap();
    let claims = Claims {
        exp: 1732212702,
        sub: user_string
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
        .map_err(InternalServerError)?;
    Ok(token)
}