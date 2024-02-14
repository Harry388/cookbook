use crate::permission::{user::{is_user, is_following_or_public}, check_exists};
use crate::api::auth::JWTAuthorization;
use poem::{Result, error::InternalServerError};
use sqlx::MySqlPool;

struct CheckAlbumResult {
    user_id: i64
}

pub async fn owns_album(pool: &MySqlPool, id: i64, auth: JWTAuthorization) -> Result<()> {
    let check_album: Option<CheckAlbumResult> = sqlx::query_as!(CheckAlbumResult,
        "select user_id from album where id = ?",
        id
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    let check_album = check_exists(check_album, "Album")?;
    is_user(check_album.user_id, auth)
}

pub async fn is_visible(pool: &MySqlPool, id: i64, auth: JWTAuthorization) -> Result<()> {
    let check_album: Option<CheckAlbumResult> = sqlx::query_as!(CheckAlbumResult,
        "select user_id from album where id = ?",
        id
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    let check_album = check_exists(check_album, "Album")?;
    is_following_or_public(pool, check_album.user_id, auth).await
}
