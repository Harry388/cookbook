use poem_openapi::Object;
use poem::{error::InternalServerError, Result};
use sqlx::{MySqlPool, types::{chrono::{DateTime, Utc}, JsonValue}};

// Inputs

#[derive(Object)]
pub struct Recipe {
    #[oai(validator(max_length=255, min_length=1))]
    title: String,
    #[oai(validator(max_length=65535))]
    description: Option<String>,
    ingredients: JsonValue,
    method: JsonValue
}

#[derive(Object)]
pub struct UpdateRecipe {
    #[oai(validator(max_length=255, min_length=1))]
    title: Option<String>,
    #[oai(validator(max_length=65535))]
    description: Option<String>,
    ingredients: Option<JsonValue>,
    method: Option<JsonValue>
}

// Results

#[derive(Object)]
pub struct RecipeResult {
    pub id: i64,
    title: String,
    description: Option<String>,
    ingredients: JsonValue,
    method: JsonValue,
    pub user_id: i64,
    user_display_name: String,
    pub created: DateTime<Utc>,
    is_liked: i64,
    pub likes: Option<i64>,
    comments: Option<i64>,
    links: Option<i64>
}

pub async fn create_recipe(pool: &MySqlPool, recipe: Recipe, auth: i64) -> Result<u64> {
    let id = sqlx::query!(
        "insert into recipe (title, description, ingredients, method, user_id)
        values (?,?,?,?,?)",
        recipe.title, recipe.description, recipe.ingredients, recipe.method, auth)
        .execute(pool)
        .await
        .map_err(InternalServerError)?
        .last_insert_id();
    Ok(id)
}

pub async fn get_recipe(pool: &MySqlPool, id: i64, auth: i64) -> Result<Option<RecipeResult>> {
    let recipe: Option<RecipeResult> = sqlx::query_as!(RecipeResult,
        "select recipe.id, title, description, ingredients, method, user_id, recipe.created, user.display_name as user_display_name,
        exists (select * from recipe_like where recipe_id = recipe.id and user_id = ?) as is_liked,
        (select count(*) from recipe_like where recipe_id = recipe.id) as likes,
        (select count(*) from recipe_comment where recipe_id = recipe.id) as comments,
        (select count(*) from recipe_post where recipe_id = recipe.id) as links
        from recipe inner join user on recipe.user_id = user.id
        where recipe.id = ?
        group by recipe.id",
        auth, id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(recipe)
}

pub async fn get_trending_recipes(pool: &MySqlPool, auth: i64) -> Result<Vec<RecipeResult>> {
    let recipes: Vec<RecipeResult> = sqlx::query_as!(RecipeResult,
        "select recipe.id, title, description, ingredients, method, recipe.user_id, recipe.created, user.display_name as user_display_name,
        exists (select * from recipe_like where recipe_id = recipe.id and user_id = ?) as is_liked,
        (select count(*) from recipe_like where recipe_id = recipe.id) as likes,
        (select count(*) from recipe_comment where recipe_id = recipe.id) as comments,
        (select count(*) from recipe_post where recipe_id = recipe.id) as links
        from recipe inner join user on recipe.user_id = user.id
        left join following on following.following_id = recipe.user_id
        inner join recipe_like on recipe_like.recipe_id = recipe.id
        where (recipe.user_id != ?) and (user.public or (following.user_id = ? and following.accepted)) and 
        (recipe_like.created > (now() - interval 1 month))
        group by recipe.id
        order by likes desc",
        auth, auth, auth)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(recipes)
}

pub async fn search_recipes(pool: &MySqlPool, search: String, auth: i64) -> Result<Vec<RecipeResult>> {
    let recipes: Vec<RecipeResult> = sqlx::query_as!(RecipeResult,
        "select recipe.id, title, description, ingredients, method, recipe.user_id, recipe.created, user.display_name as user_display_name,
        exists (select * from recipe_like where recipe_id = recipe.id and user_id = ?) as is_liked,
        (select count(*) from recipe_like where recipe_id = recipe.id) as likes,
        (select count(*) from recipe_comment where recipe_id = recipe.id) as comments,
        (select count(*) from recipe_post where recipe_id = recipe.id) as links
        from recipe inner join user on recipe.user_id = user.id
        left join following on following.following_id = recipe.user_id
        where ((recipe.user_id = ?) or user.public or (following.user_id = ? and following.accepted)) and 
        (match (title, description) against (?) or 
        exists (select * from tag inner join tag_recipe on tag.id = tag_recipe.tag_id where tag_recipe.recipe_id = recipe.id and match(tag.tag) against (?)))
        group by recipe.id
        order by created desc",
        auth, auth, auth, search, search)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(recipes)
}

pub async fn get_feed_recipes(pool: &MySqlPool, auth: i64) -> Result<Vec<RecipeResult>> {
    let recipes: Vec<RecipeResult> = sqlx::query_as!(RecipeResult,
        "select recipe.id, title, description, ingredients, method, recipe.user_id, recipe.created, user.display_name as user_display_name,
        exists (select * from recipe_like where recipe_id = recipe.id and user_id = ?) as is_liked,
        (select count(*) from recipe_like where recipe_id = recipe.id) as likes,
        (select count(*) from recipe_comment where recipe_id = recipe.id) as comments,
        (select count(*) from recipe_post where recipe_id = recipe.id) as links
        from recipe inner join user on recipe.user_id = user.id
        left join following on following.following_id = recipe.user_id
        left join tag_recipe on tag_recipe.recipe_id = recipe.id
        left join tag_user on tag_recipe.tag_id = tag_user.tag_id
        where (recipe.user_id != ?) and ((following.user_id = ? and following.accepted) or (user.public and tag_user.user_id = ?))
        group by recipe.id
        order by created desc",
        auth, auth, auth, auth)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(recipes)
}

pub async fn get_user_recipes(pool: &MySqlPool, user_id: i64, auth: i64) -> Result<Vec<RecipeResult>> {
    let recipes: Vec<RecipeResult> = sqlx::query_as!(RecipeResult,
        "select recipe.id, title, description, ingredients, method, user_id, recipe.created, user.display_name as user_display_name,
        exists (select * from recipe_like where recipe_id = recipe.id and user_id = ?) as is_liked,
        (select count(*) from recipe_like where recipe_id = recipe.id) as likes,
        (select count(*) from recipe_comment where recipe_id = recipe.id) as comments,
        (select count(*) from recipe_post where recipe_id = recipe.id) as links
        from recipe inner join user on recipe.user_id = user.id
        where user_id = ?
        group by recipe.id
        order by created desc",
        auth, user_id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(recipes)
}

pub async fn get_post_recipes(pool: &MySqlPool, id: i64, auth: i64) -> Result<Vec<RecipeResult>> {
    let recipes: Vec<RecipeResult> = sqlx::query_as!(RecipeResult,
        "select recipe.id, recipe.title, recipe.description, recipe.ingredients, recipe.method, recipe.user_id, recipe.created, user.display_name as user_display_name,
        exists (select * from recipe_like where recipe_id = recipe.id and user_id = ?) as is_liked,
        (select count(*) from recipe_like where recipe_id = recipe.id) as likes,
        (select count(*) from recipe_comment where recipe_id = recipe.id) as comments,
        (select count(*) from recipe_post where recipe_id = recipe.id) as links
        from recipe
        inner join recipe_post on recipe.id = recipe_post.recipe_id
        inner join user on recipe.user_id = user.id
        left join following on following.following_id = recipe.user_id
        where recipe_post.post_id = ? and ((recipe.user_id = ?) or (following.user_id = ? and following.accepted) or (user.public))
        group by recipe.id
        order by created desc",
        auth, id, auth, auth)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(recipes)
}

pub async fn get_album_recipes(pool: &MySqlPool, id: i64, auth: i64) -> Result<Vec<RecipeResult>> {
    let recipes: Vec<RecipeResult> = sqlx::query_as!(RecipeResult,
        "select recipe.id, recipe.title, recipe.description, recipe.ingredients, recipe.method, recipe.user_id, recipe.created, user.display_name as user_display_name,
        exists (select * from recipe_like where recipe_id = recipe.id and user_id = ?) as is_liked,
        (select count(*) from recipe_like where recipe_id = recipe.id) as likes,
        (select count(*) from recipe_comment where recipe_id = recipe.id) as comments,
        (select count(*) from recipe_post where recipe_id = recipe.id) as links
        from recipe
        inner join album_recipe on recipe.id = album_recipe.recipe_id
        inner join user on recipe.user_id = user.id
        left join following on following.following_id = recipe.user_id
        where (album_recipe.album_id = ?) and (user.public or (following.user_id = ? and following.accepted))
        group by recipe.id
        order by created desc",
        auth, id, auth)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(recipes)
}

pub async fn get_tag_recipes(pool: &MySqlPool, id: i64, auth: i64) -> Result<Vec<RecipeResult>> {
    let recipes: Vec<RecipeResult> = sqlx::query_as!(RecipeResult,
        "select recipe.id, recipe.title, recipe.description, recipe.ingredients, recipe.method, recipe.user_id, recipe.created, user.display_name as user_display_name,
        exists (select * from recipe_like where recipe_id = recipe.id and user_id = ?) as is_liked,
        (select count(*) from recipe_like where recipe_id = recipe.id) as likes,
        (select count(*) from recipe_comment where recipe_id = recipe.id) as comments,
        (select count(*) from recipe_post where recipe_id = recipe.id) as links
        from recipe
        inner join tag_recipe on recipe.id = tag_recipe.recipe_id
        inner join user on recipe.user_id = user.id
        left join following on following.following_id = recipe.user_id
        where (tag_recipe.tag_id = ?) and (user.public or (following.user_id = ? and following.accepted))
        group by recipe.id
        order by created desc",
        auth, id, auth)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(recipes)
}

pub async fn get_cookbook_section_recipes(pool: &MySqlPool, id: i64, auth: i64) -> Result<Vec<RecipeResult>> {
    let recipes: Vec<RecipeResult> = sqlx::query_as!(RecipeResult,
        "select recipe.id, recipe.title, recipe.description, recipe.ingredients, recipe.method, recipe.user_id, recipe.created, user.display_name as user_display_name,
        exists (select * from recipe_like where recipe_id = recipe.id and user_id = ?) as is_liked,
        (select count(*) from recipe_like where recipe_id = recipe.id) as likes,
        (select count(*) from recipe_comment where recipe_id = recipe.id) as comments,
        (select count(*) from recipe_post where recipe_id = recipe.id) as links
        from recipe
        inner join cookbook_recipe on recipe.id = cookbook_recipe.recipe_id
        inner join user on recipe.user_id = user.id
        left join following on following.following_id = recipe.user_id
        where (cookbook_recipe.section_id = ?) and (user.public or (recipe.user_id = ?) or (following.user_id = ? and following.accepted))
        group by recipe.id, cookbook_recipe.position
        order by cookbook_recipe.position asc",
        auth, id, auth, auth)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(recipes)
}

pub async fn update_recipe(pool: &MySqlPool, id: i64, update_recipe: UpdateRecipe) -> Result<()> {
    sqlx::query!(
        "update recipe set title = coalesce(?, title), description = coalesce(?, description),
        ingredients = coalesce(?, ingredients), method = coalesce(?, method)
        where id = ?",
        update_recipe.title, update_recipe.description, update_recipe.ingredients, update_recipe.method, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn delete_recipe(pool: &MySqlPool, id: i64) -> Result<()> {
    sqlx::query!(
        "delete from recipe where id = ?",
        id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn add_album_recipe(pool: &MySqlPool, id: i64, album_id: i64) -> Result<()> {
    sqlx::query!(
        "insert into album_recipe (album_id, recipe_id) values (?,?)",
        album_id, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn remove_album_recipe(pool: &MySqlPool, id: i64, album_id: i64) -> Result<()> {
    sqlx::query!(
        "delete from album_recipe where album_id = ? and recipe_id = ?",
        album_id, id)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn add_recipe_tags(pool: &MySqlPool, id: i64, tag_ids: Vec<i64>) -> Result<()> {
    for tag_id in tag_ids.iter() {
        sqlx::query!(
            "insert into tag_recipe (tag_id, recipe_id) values (?,?)",
            tag_id, id) 
            .execute(pool)
            .await
            .map_err(InternalServerError)?;
    }
    Ok(())
}

pub async fn remove_recipe_tags(pool: &MySqlPool, id: i64, tag_ids: Vec<i64>) -> Result<()> {
    for tag_id in tag_ids.iter() {
        sqlx::query!(
            "delete from tag_recipe where tag_id = ? and recipe_id = ?",
            tag_id, id) 
            .execute(pool)
            .await
            .map_err(InternalServerError)?;
    }
    Ok(())
}
