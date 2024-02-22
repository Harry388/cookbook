use crate::permission::{user::{is_user, is_following_or_public}, check_exists};
use crate::api::auth::JWTAuthorization;
use poem::{Result, error::InternalServerError};
use sqlx::MySqlPool;

struct CheckPostResult {
    user_id: i64,
    community_id: Option<i32>
}

pub async fn owns_post(pool: &MySqlPool, id: i64, auth: JWTAuthorization) -> Result<()> {
    let check_post: Option<CheckPostResult> = sqlx::query_as!(CheckPostResult,
        "select user_id, community_id from post where id = ?",
        id
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    let check_post = check_exists(check_post, "Post")?;
    is_user(check_post.user_id, auth)
}

pub async fn is_visible(pool: &MySqlPool, id: i64, auth: JWTAuthorization) -> Result<()> {
    let check_post: Option<CheckPostResult> = sqlx::query_as!(CheckPostResult,
        "select user_id, community_id from post where id = ?",
        id
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    let check_post = check_exists(check_post, "Post")?;
    if let Some(_) = check_post.community_id {
        return Ok(());
    }
    is_following_or_public(pool, check_post.user_id, auth).await
}
