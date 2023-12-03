use poem::{Result, error::Error, http::StatusCode};
use crate::api::auth::JWTAuthorization;

fn create_permission_error() -> Error {
    Error::from_string("Unauthorized", StatusCode::UNAUTHORIZED)
}

pub fn is_user(id: i64, auth: JWTAuthorization) -> Result<()> {
    if id == auth.0 {
        Ok(())
    }
    else {
        Err(create_permission_error())
    }
}