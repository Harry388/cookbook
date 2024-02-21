use poem_openapi::Union;
use crate::model::{post, recipe};

#[derive(Union)]
#[oai(discriminator_name = "type")]
pub enum Entry {
    Post(post::PostResult),
    Recipe(recipe::RecipeResult)
}

pub fn create_entries(posts: Vec<post::PostResult>, recipes: Vec<recipe::RecipeResult>) -> Vec<Entry> {
    let mut entries: Vec<Entry> = posts.into_iter().map(|p| Entry::Post(p)).collect();
    let recipes: Vec<Entry> = recipes.into_iter().map(|r| Entry::Recipe(r)).collect();
    entries.extend(recipes);
    entries
}
