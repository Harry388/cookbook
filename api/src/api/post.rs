use poem_openapi::{OpenApi, payload::{Json, PlainText}, Object, ApiResponse, param::Path, Tags};
use poem::{web::Data, error::InternalServerError, Result};
use sqlx::MySqlPool;
use crate::api::auth::JWTAuthorization;
use crate::permission;

#[derive(Tags)]
enum ApiTags {
    Post
}

// Inputs



// Results



// Responses



pub struct PostApi;

#[OpenApi(prefix_path = "/post", tag = "ApiTags::Post")]
impl PostApi {
    
    

}