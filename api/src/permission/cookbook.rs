use crate::permission::{user::{is_user, is_following_or_public}, check_exists};
use crate::api::auth::JWTAuthorization;
use poem::{Result, error::InternalServerError};
use sqlx::MySqlPool;

struct CheckCookbookResult {
    user_id: i64
}

pub async fn owns_cookbook(pool: &MySqlPool, id: i64, auth: JWTAuthorization) -> Result<()> {
    let check_cookbook: Option<CheckCookbookResult> = sqlx::query_as!(CheckCookbookResult,
        "select user_id from cookbook where id = ?",
        id
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    let check_cookbook = check_exists(check_cookbook, "Cookbook")?;
    is_user(check_cookbook.user_id, auth)
}

pub async fn is_visible(pool: &MySqlPool, id: i64, auth: JWTAuthorization) -> Result<()> {
    let check_cookbook: Option<CheckCookbookResult> = sqlx::query_as!(CheckCookbookResult,
        "select user_id from cookbook where id = ?",
        id
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    let check_cookbook = check_exists(check_cookbook, "Cookbook")?;
    is_following_or_public(pool, check_cookbook.user_id, auth).await
}
