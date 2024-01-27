use crate::permission::check_permission_option;
use crate::api::auth::JWTAuthorization;
use poem::{Result, error::InternalServerError};
use sqlx::MySqlPool;

pub async fn is_in(pool: &MySqlPool, id: i64, auth: JWTAuthorization) -> Result<()> {
    let check_in = sqlx::query!(
        "select * from community_user where community_id = ? and user_id = ?",
        id, auth.0
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    check_permission_option(check_in)
}

pub async fn is_admin(pool: &MySqlPool, id: i64, auth: JWTAuthorization) -> Result<()> {
    let check_in = sqlx::query!(
        "select * from community_user where community_id = ? and user_id = ? and permission = 'ADMIN'",
        id, auth.0
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    check_permission_option(check_in)
}
