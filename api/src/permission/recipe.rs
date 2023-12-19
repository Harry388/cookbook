use crate::permission::{user::is_user, create_not_found_error};
use crate::api::auth::JWTAuthorization;
use poem::{Result, error::InternalServerError};
use sqlx::MySqlPool;

struct CheckRecipeResult {
    user_id: i64
}

pub async fn owns_recipe(pool: &MySqlPool, id: i64, auth: JWTAuthorization) -> Result<()> {
    let check_recipe: Option<CheckRecipeResult> = sqlx::query_as!(CheckRecipeResult,
        "select user_id from recipe where id = ?",
        id
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    if let None = check_recipe {
        return Err(create_not_found_error("Recipe"));
    }
    let check_recipe = check_recipe.unwrap();
    is_user(check_recipe.user_id, auth)
}