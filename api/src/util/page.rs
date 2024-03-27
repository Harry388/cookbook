use poem_openapi::Union;
use crate::model::{cookbook, recipe};

#[derive(Union)]
#[oai(discriminator_name = "type")]
pub enum Page {
    Section(cookbook::SectionResult),
    Recipe(recipe::RecipeResult)
}
