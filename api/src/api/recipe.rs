use poem_openapi::{OpenApi, payload::{Json, PlainText}, Object, ApiResponse, param::Path, Tags};
use poem::{web::Data, error::InternalServerError, Result};
use sqlx::{MySqlPool, types::chrono::{DateTime, Utc}};
use crate::api::auth::JWTAuthorization;
use crate::permission;

#[derive(Tags)]
enum ApiTags {
    Recipe
}

// Inputs



// Results



// Responses



pub struct RecipeApi;

#[OpenApi(prefix_path = "/recipe", tag = "ApiTags::Recipe")]
impl RecipeApi {
    
    

}