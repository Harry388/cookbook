use poem_openapi::Object;
use poem::{error::InternalServerError, Result};
use sqlx::MySqlPool;

// Inputs

#[derive(Object)]
pub struct Album {
    title: String
}

// Results

#[derive(Object)]
pub struct AlbumResult {
    id: i64,
    title: String,
    user_id: i64
}

pub async fn create_album(pool: &MySqlPool, album: Album, auth: i64) -> Result<()> {
    sqlx::query!(
        "insert into album (title, user_id) values (?,?)",
        album.title, auth)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn get_album(pool: &MySqlPool, id: i64) -> Result<Option<AlbumResult>> {
    let album = sqlx::query_as!(AlbumResult,
        "select id, title, user_id
        from album
        where id = ?",
        id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(album)
}

pub async fn get_user_albums(pool: &MySqlPool, user_id: i64) -> Result<Vec<AlbumResult>> {
    let albums = sqlx::query_as!(AlbumResult,
        "select id, title, user_id
        from album
        where user_id = ?
        order by title",
        user_id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(albums)
}

pub async fn update_album(pool: &MySqlPool, id: i64, album: Album) -> Result<()> {
    sqlx::query!(
        "update album set title = coalesce(?, title) where id = ?",
        album.title, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn delete_album(pool: &MySqlPool, id: i64) -> Result<()> {
    sqlx::query!(
        "delete from album where id = ?",
        id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}
