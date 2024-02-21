use poem_openapi::{payload::{Attachment, AttachmentType}, Object, types::{Email, multipart::Upload}, Multipart};
use poem::{Result, error::InternalServerError};
use sqlx::{MySqlPool, types::chrono::{DateTime, Utc}};
use crate::api::auth::generate_password_hash;
use crate::storage::Storage;

// Inputs

#[derive(Object)]
pub struct User {
    username: String,
    display_name: String,
    email: Email,
    password: String,
    bio: Option<String>
}

#[derive(Object)]
pub struct UpdateUser {
    display_name: Option<String>,
    bio: Option<String>,
    username: Option<String>
}

#[derive(Multipart)]
pub struct SetProfilePic {
    pic: Upload
}

// Results

#[derive(Object)]
pub struct UserResult {
    id: i64,
    username: String,
    display_name: String,
    bio: Option<String>,
    public: i8,
    following: i64,
    followers: i64,
    is_following: Option<f32>,
    created: DateTime<Utc>
}

#[derive(Object)]
pub struct FollowResult {
    id: i64,
    username: String,
    display_name: String,
    pfp: Option<String>
}

struct GetProfilePicResult {
    pfp: Option<String>
}

pub enum ProfilePicResult {
    Ok(Attachment<Vec<u8>>),
    UserNotFound,
    PicNotFound
}

#[derive(Object)]
pub struct CommunityUserResult {
    id: i64,
    username: String,
    display_name: String,
    permission: String
}

pub async fn create_user(pool: &MySqlPool, user: User) -> Result<u64> {
    let password = generate_password_hash(&user.password)?;
    let id = sqlx::query!( 
        "insert into user (username, display_name, email, password, bio, public)
        values (?,?,?,?,?,?)",
        user.username, user.display_name, user.email.0, password, user.bio, true)
        .execute(pool)
        .await
        .map_err(InternalServerError)?
        .last_insert_id();
    Ok(id) 
}

pub async fn get_user(pool: &MySqlPool, id: i64, auth: i64) -> Result<Option<UserResult>> {
    let user = sqlx::query_as!(UserResult,
        "with user_and_followers as (
            select id, username, display_name, bio, public, created,
            count(followers.following_id) as followers,
            cast(sum(case when followers.user_id = ? then 1 else 0 end) as float) as is_following
            from user
            left join following as followers on user.id = followers.following_id
            where id = ?
            group by id
        )
        select id, username, display_name, bio, public, followers, is_following, created,
        count(following.following_id) as following
        from user_and_followers
        left join following on user_and_followers.id = following.user_id
        group by id",
        auth, id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(user)
}

pub async fn update_user(pool: &MySqlPool, id: i64, user: UpdateUser) -> Result<()> { 
    sqlx::query!(
        "update user set username = coalesce(?, username), display_name = coalesce(?, display_name), bio = coalesce(?, bio) where id = ?",
        user.username, user.display_name, user.bio, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn set_profile_pic(pool: &MySqlPool, storage: &dyn Storage, id: i64, pfp: SetProfilePic) -> Result<()> {
    let current_pfp: Option<GetProfilePicResult> = sqlx::query_as!(GetProfilePicResult,
        "select pfp from user where id = ?",
        id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    if let Some(cpfp) = current_pfp {
        if let Some(current_path) = cpfp.pfp {
            storage.delete_file(&current_path).await?;
        }
    }
    let path = format!("user/{}/pfp", id);
    let pfp_path = storage.put_file(&path, pfp.pic).await?;
    sqlx::query!(
        "update user set pfp = ? where id = ?",
        pfp_path, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn get_profile_pic(pool: &MySqlPool, storage: &dyn Storage, id: i64) -> Result<ProfilePicResult> {
    let pfp: Option<GetProfilePicResult> = sqlx::query_as!(GetProfilePicResult,
        "select pfp from user where id = ?",
        id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    if let None = pfp {
        return Ok(ProfilePicResult::UserNotFound);
    }
    if let None = pfp.as_ref().unwrap().pfp {
        return Ok(ProfilePicResult::PicNotFound);
    }
    let pfp_path = pfp.unwrap().pfp.unwrap();
    let pic = storage.get_file(&pfp_path).await?;
    let attachment = 
        Attachment::new(pic)
        .attachment_type(AttachmentType::Inline)
        .filename(pfp_path);
    Ok(ProfilePicResult::Ok(attachment))
}

pub async fn delete_user(pool: &MySqlPool, storage: &dyn Storage, id: i64) -> Result<()> {
    sqlx::query!(
        "delete from user where id = ?",
        id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    storage.delete_file(&format!("user/{}", id)).await?;
    Ok(())
}

pub async fn follow(pool: &MySqlPool, id: i64, follow_id: i64) -> Result<()> {
    sqlx::query!(
        "insert into following (user_id, following_id) values (?, ?)",
        id, follow_id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn unfollow(pool: &MySqlPool, id: i64, follow_id: i64) -> Result<()> {
    sqlx::query!(
        "delete from following where user_id = ? and following_id = ?",
        id, follow_id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn search_users(pool: &MySqlPool, search: String) -> Result<Vec<FollowResult>> {
    let search = format!("%{search}%");
    let users = sqlx::query_as!(FollowResult,
        "select id, username, display_name, pfp from user 
        where (username like ?) or (display_name like ?)
        order by display_name",
        search, search)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(users)
}

pub async fn get_followers(pool: &MySqlPool, id: i64) -> Result<Vec<FollowResult>> {
    let followers = sqlx::query_as!(FollowResult,
        "select id, username, display_name, pfp from user where id in (
            select user_id from following where following_id = ?
        )
        order by display_name",
        id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(followers)
}

pub async fn get_following(pool: &MySqlPool, id: i64) -> Result<Vec<FollowResult>> {
    let following = sqlx::query_as!(FollowResult,
        "select id, username, display_name, pfp from user where id in (
            select following_id from following where user_id = ?
        )
        order by display_name",
        id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(following)
}

pub async fn get_community_users(pool: &MySqlPool, id: i64) -> Result<Vec<CommunityUserResult>> {
    let users = sqlx::query_as!(CommunityUserResult,
        "select id, username, display_name, permission 
        from user inner join community_user on user.id = community_user.user_id
        where community_user.community_id = ?
        group by user.id, permission
        order by display_name",
        id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(users)
}
