use crate::permission::{user::is_user, check_exists};
use crate::api::auth::JWTAuthorization;
use poem::{Result, error::InternalServerError};
use sqlx::MySqlPool;

struct CheckCommentResult {
    user_id: i64
}

pub async fn owns_comment(pool: &MySqlPool, id: i64, auth: JWTAuthorization) -> Result<()> {
    let check_comment: Option<CheckCommentResult> = sqlx::query_as!(CheckCommentResult,
        "select user_id from comment where id = ?",
        id
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    let check_comment = check_exists(check_comment, "Comment")?;
    is_user(check_comment.user_id, auth)
}
