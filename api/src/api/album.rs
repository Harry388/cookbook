use poem_openapi::{OpenApi, payload::Json, param::Path, Tags, ApiResponse};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use futures::try_join;
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::model::{album, post, recipe};
use crate::util::entry;

#[derive(Tags)]
enum ApiTags {
    Album
}

// Responses

#[derive(ApiResponse)]
enum GetAlbumResponse {
    #[oai(status = 200)]
    Ok(Json<album::AlbumResult>),
    #[oai(status = 404)]
    NotFound
}

#[derive(ApiResponse)]
enum GetUserAlbumsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<album::AlbumResult>>)
}

#[derive(ApiResponse)]
enum GetAlbumEntriesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<entry::Entry>>)
}

#[derive(ApiResponse)]
enum AlbumEntryResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 400)]
    BadRequest
}

pub struct AlbumApi;

#[OpenApi(prefix_path = "/album", tag = "ApiTags::Album")]
impl AlbumApi {

    #[oai(path = "/", method = "post")]
    async fn create_album(&self, pool: Data<&MySqlPool>, album: Json<album::Album>, auth: JWTAuthorization) -> Result<()> {
        album::create_album(pool.0, album.0, auth.0).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_album(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetAlbumResponse> {
        permission::album::is_visible(pool.0, id.0, auth).await?;
        let album = album::get_album(pool.0, id.0).await?;
        Ok(
            match album {
                Some(a) => GetAlbumResponse::Ok(Json(a)),
                None => GetAlbumResponse::NotFound
            }
        )
    }

    #[oai(path = "/user/:user_id", method = "get")]
    async fn get_user_albums(&self, pool: Data<&MySqlPool>, user_id: Path<i64>, auth: JWTAuthorization) -> Result<GetUserAlbumsResponse>{
        permission::user::is_following_or_public(pool.0, user_id.0, auth).await?;
        let albums = album::get_user_albums(pool.0, user_id.0).await?;
        Ok(GetUserAlbumsResponse::Ok(Json(albums)))
    }

    #[oai(path = "/:id", method = "put")]
    async fn update_album(&self, pool: Data<&MySqlPool>, id: Path<i64>, album: Json<album::Album>, auth: JWTAuthorization) -> Result<()> {
        permission::album::owns_album(pool.0, id.0, auth).await?;
        album::update_album(pool.0, id.0, album.0).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_album(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::album::owns_album(pool.0, id.0, auth).await?;
        album::delete_album(pool.0, id.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/contents", method = "get")]
    async fn get_album_entries(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetAlbumEntriesResponse> {
        permission::album::is_visible(pool.0, id.0, auth).await?;
        let posts_fut = post::get_album_posts(pool.0, id.0, auth.0);
        let recipes_fut = recipe::get_album_recipes(pool.0, id.0, auth.0);
        let (posts, recipes) = try_join!(posts_fut, recipes_fut)?;
        let entries = entry::create_entries(posts, recipes);
        Ok(GetAlbumEntriesResponse::Ok(Json(entries)))
    }

    #[oai(path = "/:id/:entry/:entry_id", method = "post")]
    async fn add_album_entry(&self, pool: Data<&MySqlPool>, id: Path<i64>, entry: Path<String>, entry_id: Path<i64>, auth: JWTAuthorization) -> Result<AlbumEntryResponse> {
        permission::album::owns_album(pool.0, id.0, auth).await?;
        match entry.0.as_str() {
           "post" => {
               permission::post::is_visible(pool.0, entry_id.0, auth).await?;
               post::add_album_post(pool.0, entry_id.0, id.0).await?;
           },
           "recipe" => {
               permission::recipe::is_visible(pool.0, entry_id.0, auth).await?;
               recipe::add_album_recipe(pool.0, entry_id.0, id.0).await?;
           },
           _ => return Ok(AlbumEntryResponse::BadRequest)
        }
        Ok(AlbumEntryResponse::Ok)
    }

    #[oai(path = "/:id/:entry/:entry_id", method = "delete")]
    async fn delete_album_entry(&self, pool: Data<&MySqlPool>, id: Path<i64>, entry: Path<String>, entry_id: Path<i64>, auth: JWTAuthorization) -> Result<AlbumEntryResponse> {
        permission::album::owns_album(pool.0, id.0, auth).await?;
        match entry.0.as_str() {
           "post" => post::remove_album_post(pool.0, entry_id.0, id.0).await?,
           "recipe" => recipe::remove_album_recipe(pool.0, entry_id.0, id.0).await?,
           _ => return Ok(AlbumEntryResponse::BadRequest)
        }
        Ok(AlbumEntryResponse::Ok)
    }

}
