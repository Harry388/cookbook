use poem_openapi::Union;
use crate::model::{post, recipe};

#[derive(Union)]
#[oai(discriminator_name = "type")]
pub enum Entry {
    Post(post::PostResult),
    Recipe(recipe::RecipeResult)
}

pub enum OrderBy {
    Created,
    Likes
}

impl OrderBy {

    fn compare(&self, a: &Entry, b: &Entry) -> std::cmp::Ordering {
        let ordering = match self {
            OrderBy::Created => {
                let a_created = match a {
                    Entry::Post(post) => post.created,
                    Entry::Recipe(recipe) => recipe.created
                };
                let b_created = match b {
                    Entry::Post(post) => post.created,
                    Entry::Recipe(recipe) => recipe.created
                };
                b_created.partial_cmp(&a_created)
            },
            OrderBy::Likes => {
                 let a_likes = match a {
                    Entry::Post(post) => post.likes,
                    Entry::Recipe(recipe) => recipe.likes
                };
                let b_likes = match b {
                    Entry::Post(post) => post.likes,
                    Entry::Recipe(recipe) => recipe.likes
                };
                b_likes.partial_cmp(&a_likes)
           }
        };
        match ordering {
            Some(o) => o,
            None => std::cmp::Ordering::Equal
        }
    }

}

pub fn create_entries(posts: Vec<post::PostResult>, recipes: Vec<recipe::RecipeResult>, order_by: OrderBy) -> Vec<Entry> {
    let mut entries: Vec<Entry> = posts.into_iter().map(|p| Entry::Post(p)).collect();
    let recipes: Vec<Entry> = recipes.into_iter().map(|r| Entry::Recipe(r)).collect();
    entries.extend(recipes);
    entries.sort_by(|a, b| order_by.compare(a, b));
    entries
}
