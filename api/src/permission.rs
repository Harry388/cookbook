use poem::{Result, error::{Error, InternalServerError}, http::StatusCode};
use crate::api::auth::JWTAuthorization;
use sqlx::MySqlPool;

fn create_permission_error() -> Error {
    Error::from_string("Unauthorized", StatusCode::UNAUTHORIZED)
}

fn check_permission_option<T>(option: Option<T>) -> Result<()> {
    match option {
        Some(_) => Ok(()),
        None => Err(create_permission_error())
    }
}

fn check_permission_condition(condition: bool) -> Result<()> {
    if condition {
        Ok(())
    }
    else {
        Err(create_permission_error())
    }
}

pub fn is_user(id: i64, auth: JWTAuthorization) -> Result<()> {
    check_permission_condition(id == auth.0)
}

pub fn in_user_list(ids: &Vec<i64>, auth: JWTAuthorization) -> Result<()> {
    for id in ids.iter() {
        if *id == auth.0 {
            return Ok(())
        }
    }
    Err(create_permission_error())
}

pub async fn is_following(pool: &MySqlPool, id: i64, auth: JWTAuthorization) -> Result<()> {
    if id == auth.0 { return Ok(()) }
    let following = sqlx::query!(
        "select * from following where user_id = ? and following_id = ?",
        auth.0, id
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    check_permission_option(following)
}

pub async fn is_public(pool: &MySqlPool, id: i64, auth: JWTAuthorization) -> Result<()> {
    if id == auth.0 { return Ok(()) }
    let public = sqlx::query!(
        "select * from user where id = ? and public = ?",
        id, true
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    check_permission_option(public)
}

pub async fn is_following_or_public(pool: &MySqlPool, id: i64, auth: JWTAuthorization) -> Result<()> {
    if id == auth.0 { return Ok(()) }
    let following_or_public = sqlx::query!(
        "select * from user where id = ? and ((public = ?) or (id in (
            select following_id from following where user_id = ? and following_id = ?
        )))",
        id, true, auth.0, id
        )
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    check_permission_option(following_or_public)
}