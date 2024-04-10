use poem_openapi::Object;
use poem::{error::InternalServerError, Result};
use sqlx::{MySqlPool, types::chrono::{DateTime, Utc}};

// Inputs

#[derive(Object)]
pub struct Community {
    #[oai(validator(max_length=255, min_length=1))]
    title: String,
    #[oai(validator(max_length=65535))]
    description: Option<String>
}

#[derive(Object)]
pub struct UpdateCommunity {
    #[oai(validator(max_length=255, min_length=1))]
    title: Option<String>,
    #[oai(validator(max_length=65535))]
    description: Option<String>,
    public: bool
}

#[derive(Object)]
pub struct UpdateCommunityUser {
    #[oai(validator(pattern="ADMIN|USER"))]
    permission: String 
}

// Results

#[derive(Object)]
pub struct CommunityResult {
    id: i64,
    title: String,
    description: Option<String>,
    created: DateTime<Utc>,
    users: i64,
    is_member: Option<f32>,
    is_requested: i64,
    is_admin: Option<f32>,
    public: i8
}

struct UserCountResult {
    count: i64
}

pub async fn create_community(pool: &MySqlPool, community: Community, auth: i64) -> Result<()> {
    let community_id = sqlx::query!(
        "insert into community (title, description, public) values (?,?,?)",
        community.title, community.description, true)
        .execute(pool)
        .await
        .map_err(InternalServerError)?
        .last_insert_id();
    sqlx::query!(
        "insert into community_user (community_id, user_id, permission, accepted)
        values (?, ?, 'ADMIN', ?)",
        community_id, auth, true)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn get_community(pool: &MySqlPool, id: i64, auth: i64) -> Result<Option<CommunityResult>> {
    let community = sqlx::query_as!(CommunityResult,
        "select id, title, description, created, count(*) as users, public,
        cast(sum(case when community_user.user_id = ? then 1 else 0 end) as float) as is_member,
        cast(sum(case when community_user.user_id = ? and community_user.permission = 'ADMIN' then 1 else 0 end) as float) as is_admin,
        exists (select * from community_user where user_id = ? and community_id = ? and not accepted) as is_requested
        from community
        inner join community_user on community.id = community_user.community_id and community_user.accepted
        where community.id = ?
        group by community.id",
        auth, auth, auth, id, id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(community)
}

pub async fn join_community(pool: &MySqlPool, id: i64, accepted: bool, auth: i64) -> Result<()> {
    sqlx::query!(
        "insert into community_user (community_id, user_id, permission, accepted) 
        values (?, ?, 'USER', ?)",
        id, auth, accepted)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn update_community(pool: &MySqlPool, id: i64, update: UpdateCommunity) -> Result<()> {
    sqlx::query!(
        "update community set title = coalesce(?, title), description = coalesce(?, description), public = coalesce(?, public)
        where id = ?",
        update.title, update.description, update.public, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    if update.public {
        accept_all_requests(pool, id).await?;
    }
    Ok(())
}

async fn accept_all_requests(pool: &MySqlPool, id: i64) -> Result<()> {
    sqlx::query!(
        "update community_user set accepted = ? where community_id = ?",
        true, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn delete_community(pool: &MySqlPool, id: i64) -> Result<()> {
    sqlx::query!(
        "delete from community where id = ?",
        id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn search_communities(pool: &MySqlPool, search: String, auth: i64) -> Result<Vec<CommunityResult>> {
    let search = format!("%{search}%");
    let communities = sqlx::query_as!(CommunityResult,
        "with community_and_users as (
            select id, title, description, created, count(*) as users, public,
            cast(sum(case when community_user.user_id = ? then 1 else 0 end) as float) as is_member,
            cast(sum(case when community_user.user_id = ? and community_user.permission = 'ADMIN' then 1 else 0 end) as float) as is_admin,
            exists (select * from community_user where user_id = ? and community_id = id and not accepted) as is_requested
            from community
            inner join community_user on community.id = community_user.community_id and community_user.accepted
            group by community.id
        )
        select id, title, description, created, users, is_member, is_admin, public, is_requested
        from community_and_users
        inner join community_user on community_user.community_id = community_and_users.id
        where (title like ?) or (description like ?)
        group by id
        order by title",
        auth, auth, auth, search, search)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(communities)
}

pub async fn get_user_communities(pool: &MySqlPool, user_id: i64, auth: i64) -> Result<Vec<CommunityResult>> {
    let communities = sqlx::query_as!(CommunityResult,
        "with community_and_users as (
            select id, title, description, created, count(*) as users, public,
            cast(sum(case when community_user.user_id = ? then 1 else 0 end) as float) as is_member,
            cast(sum(case when community_user.user_id = ? and community_user.permission = 'ADMIN' then 1 else 0 end) as float) as is_admin
            from community
            inner join community_user on community.id = community_user.community_id and community_user.accepted
            group by community.id
        )
        select id, title, description, created, users, is_member, is_admin, public, 0 as is_requested
        from community_and_users
        inner join community_user on community_user.community_id = community_and_users.id and community_user.accepted
        where community_user.user_id = ?
        order by title",
        auth, auth, user_id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(communities)
}

pub async fn update_community_user(pool: &MySqlPool, id: i64, user_id: i64, update: UpdateCommunityUser) -> Result<()> {
    sqlx::query!(
        "update community_user set permission = ?
        where community_id = ? and user_id = ?",
        update.permission, id, user_id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn leave_community(pool: &MySqlPool, id: i64, user_id: i64) -> Result<()> {
    sqlx::query!(
        "delete from community_user where community_id = ? and user_id = ?",
        id, user_id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn has_one_admin(pool: &MySqlPool, id: i64) -> Result<bool> {
    let admin_count = sqlx::query_as!(UserCountResult,
        "select count(*) as count
        from community
        inner join community_user on community.id = community_user.community_id
        where community.id = ? and community_user.permission = 'ADMIN'
        group by community.id",
        id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(
        match admin_count {
            Some(uc) => uc.count == 1,
            None => true
        }
    )
}

pub async fn accept_member(pool: &MySqlPool, id: i64, user_id: i64) -> Result<()> {
    sqlx::query!(
        "update community_user set accepted = ? where community_id = ? and user_id = ?",
        true, id, user_id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}
