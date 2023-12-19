pub mod user;
pub mod recipe;

use poem::{Result, http::StatusCode, error::Error};

pub fn create_permission_error() -> Error {
    Error::from_string("Unauthorized", StatusCode::UNAUTHORIZED)
}

pub fn create_not_found_error(item: &str) -> Error {
    let msg = &format!("{} not found", item);
    Error::from_string(msg, StatusCode::NOT_FOUND)
}

pub fn check_permission_option<T>(option: Option<T>) -> Result<()> {
    match option {
        Some(_) => Ok(()),
        None => Err(create_permission_error())
    }
}

pub fn check_permission_condition(condition: bool) -> Result<()> {
    if condition {
        Ok(())
    }
    else {
        Err(create_permission_error())
    }
}