use crate::permission::check_permission_option;
use crate::api::auth::JWTAuthorization;
use poem::{Result, error::InternalServerError};
use sqlx::MySqlPool;

pub async fn is_public(pool: &MySqlPool, id: i64) -> Result<()> {
    let public = sqlx::query!(
        "select * from community where id = ? and public = ?",
        id, true
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    check_permission_option(public)
}

pub async fn is_in(pool: &MySqlPool, id: i64, auth: JWTAuthorization) -> Result<()> {
    let check_in = sqlx::query!(
        "select * from community_user where community_id = ? and user_id = ? and accepted",
        id, auth.0
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    check_permission_option(check_in)
}

pub async fn is_admin(pool: &MySqlPool, id: i64, auth: JWTAuthorization) -> Result<()> {
    let check_in = sqlx::query!(
        "select * from community_user where community_id = ? and user_id = ? and accepted and permission = 'ADMIN'",
        id, auth.0
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    check_permission_option(check_in)
}
