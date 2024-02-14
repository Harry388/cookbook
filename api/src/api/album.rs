use poem_openapi::{OpenApi, payload::Json, param::Path, Tags, ApiResponse};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::model::album;

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

    #[oai(path="/:id", method = "delete")]
    async fn delete_album(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::album::owns_album(pool.0, id.0, auth).await?;
        album::delete_album(pool.0, id.0).await?;
        Ok(())
    }

}
