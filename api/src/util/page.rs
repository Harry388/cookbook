use poem_openapi::{Union, Object};
use crate::model::{cookbook, recipe};

#[derive(Union)]
#[oai(discriminator_name = "type")]
pub enum Page {
    Section(cookbook::SectionResult),
    Recipe(recipe::RecipeResult),
    Image(ImagePage)
}

#[derive(Object)]
pub struct ImagePage {
    pub image: String
}
