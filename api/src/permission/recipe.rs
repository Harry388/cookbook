use crate::permission::{user::is_user, check_exists};
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
    let check_recipe = check_exists(check_recipe, "Recipe")?;
    is_user(check_recipe.user_id, auth)
}