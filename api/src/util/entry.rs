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
    entries.sort_by(|a, b| {
        let a_created = match a {
            Entry::Post(post) => post.created,
            Entry::Recipe(recipe) => recipe.created
        };
        let b_created = match b {
            Entry::Post(post) => post.created,
            Entry::Recipe(recipe) => recipe.created
        };
        let ordering = a_created.partial_cmp(&b_created);
        match ordering {
            Some(o) => o,
            None => std::cmp::Ordering::Equal
        }
    });
    entries
}
