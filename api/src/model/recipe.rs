use poem_openapi::Object;
use poem::{error::InternalServerError, Result};
use sqlx::{MySqlPool, types::{chrono::{DateTime, Utc}, JsonValue}};

// Inputs

#[derive(Object)]
pub struct Recipe {
    title: String,
    description: Option<String>,
    ingredients: JsonValue,
    method: JsonValue
}

#[derive(Object)]
pub struct UpdateRecipe {
    title: Option<String>,
    description: Option<String>,
    ingredients: Option<JsonValue>,
    method: Option<JsonValue>
}

// Results

#[derive(Object)]
pub struct RecipeResult {
    id: i64,
    title: String,
    description: Option<String>,
    ingredients: JsonValue,
    method: JsonValue,
    pub user_id: i64,
    created: DateTime<Utc>
}

pub async fn create_recipe(pool: &MySqlPool, recipe: Recipe, auth: i64) -> Result<()> {
    sqlx::query!(
        "insert into recipe (title, description, ingredients, method, user_id)
        values (?,?,?,?,?)",
        recipe.title, recipe.description, recipe.ingredients, recipe.method, auth)
        .execute(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(())
}

pub async fn get_recipe(pool: &MySqlPool, id: i64) -> Result<Option<RecipeResult>> {
    let recipe: Option<RecipeResult> = sqlx::query_as!(RecipeResult,
        "select id, title, description, ingredients, method, user_id, created from recipe where id = ?",
        id)
        .fetch_optional(pool)
        .await
        .map_err(InternalServerError)?;
    if let None = recipe {
        return Ok(None);
    }
    let recipe = recipe.unwrap();
    Ok(Some(recipe))
}

pub async fn get_user_recipes(pool: &MySqlPool, user_id: i64) -> Result<Vec<RecipeResult>> {
    let recipes: Vec<RecipeResult> = sqlx::query_as!(RecipeResult,
        "select id, title, description, ingredients, method, user_id, created from recipe where user_id = ?",
        user_id)
        .fetch_all(pool)
        .await
        .map_err(InternalServerError)?;
    Ok(recipes)
}

pub async fn get_post_recipes(pool: &MySqlPool, id: i64) -> Result<Vec<RecipeResult>> {
    let recipes: Vec<RecipeResult> = sqlx::query_as!(RecipeResult,
        "select recipe.id, recipe.title, recipe.description, recipe.ingredients, recipe.method, recipe.user_id, recipe.created
        from recipe
        inner join recipe_post on recipe.id = recipe_post.recipe_id
        where recipe_post.post_id = ?",
        id)
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
