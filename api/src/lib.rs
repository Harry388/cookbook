pub mod api {
    pub mod user;
    pub mod auth;
    pub mod post;
    pub mod recipe;
    pub mod community;
    pub mod comment;
    pub mod album;
    pub mod tag;
    pub mod search;
}

pub mod model {
    pub mod user;
    pub mod post;
    pub mod recipe;
    pub mod community;
    pub mod comment;
    pub mod album;
    pub mod tag;
    pub mod like;
    pub mod cookbook;
}

pub mod permission;

pub mod storage;

pub mod util {
    pub mod entry;
}
