use poem_openapi::payload::{Attachment, PlainText};
use poem_openapi::{OpenApi, payload::Json, param::Path, Tags, ApiResponse};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use futures::try_join;
use crate::api::auth::JWTAuthorization;
use crate::permission;
use crate::model::{cookbook, recipe};
use crate::storage::dufs::DufsStorage;
use crate::util::page;

#[derive(Tags)]
enum ApiTags {
    Cookbook
}

// Responses

#[derive(ApiResponse)]
enum GetCookbookResponse {
    #[oai(status = 200)]
    Ok(Json<cookbook::CookbookResult>),
    #[oai(status = 404)]
    NotFound
}

#[derive(ApiResponse)]
enum GetUserCookbookResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<cookbook::CookbookResult>>)
}

#[derive(ApiResponse)]
enum GetRecipePicResponse {
    #[oai(status = 200)]
    Ok(Attachment<Vec<u8>>),
    #[oai(status = 404)]
    NotFound(PlainText<String>)
}

type GetCookbookPagesResponse = Json<Vec<page::Page>>;

pub struct CookbookApi;

#[OpenApi(prefix_path = "/cookbook", tag = "ApiTags::Cookbook")]
impl CookbookApi {

    #[oai(path = "/", method = "post")]
    async fn create_cookbook(&self, pool: Data<&MySqlPool>, cookbook: Json<cookbook::Cookbook>, auth: JWTAuthorization) -> Result<()> {
        cookbook::create_cookbook(pool.0, cookbook.0, auth.0).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_cookbook(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetCookbookResponse> {
        permission::cookbook::is_visible(pool.0, id.0, auth).await?;
        let cookbook = cookbook::get_cookbook(pool.0, id.0).await?;
        Ok(
            match cookbook {
                Some(cb) => GetCookbookResponse::Ok(Json(cb)),
                None => GetCookbookResponse::NotFound
            }
        )
    }

    #[oai(path = "/user/:user_id", method = "get")]
    async fn get_user_cookbooks(&self, pool: Data<&MySqlPool>, user_id: Path<i64>, auth: JWTAuthorization) -> Result<GetUserCookbookResponse> {
        permission::user::is_following_or_public(pool.0, user_id.0, auth).await?;
        let cookbooks = cookbook::get_user_cookbooks(pool.0, user_id.0).await?;
        Ok(GetUserCookbookResponse::Ok(Json(cookbooks)))
    }

    #[oai(path = "/:id", method = "put")]
    async fn update_cookbook(&self, pool: Data<&MySqlPool>, id: Path<i64>, cookbook: Json<cookbook::Cookbook>, auth: JWTAuthorization) -> Result<()> {
        permission::cookbook::owns_cookbook(pool.0, id.0, auth).await?;
        cookbook::update_cookbook(pool.0, id.0, cookbook.0).await?;
        Ok(())
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete_cookbook(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::cookbook::owns_cookbook(pool.0, id.0, auth).await?;
        cookbook::delete_cookbook(pool.0, storage.0, id.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/section/:section_id/recipe/:recipe_id", method = "post")]
    async fn add_recipe(&self, pool: Data<&MySqlPool>, id: Path<i64>, section_id: Path<i64>, recipe_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        let owns_cookbook = permission::cookbook::owns_cookbook(pool.0, id.0, auth);
        let owns_recipe = permission::recipe::owns_recipe(pool.0, recipe_id.0, auth);
        try_join!(owns_cookbook, owns_recipe)?;
        let recipes = recipe::get_cookbook_section_recipes(pool.0, section_id.0, auth.0).await?;
        cookbook::add_recipe(pool.0, section_id.0, recipe_id.0, recipes.len().try_into().unwrap()).await?;
        Ok(())
    }

    #[oai(path = "/:id/section/:section_id/recipe/:recipe_id", method = "delete")]
    async fn remove_recipe(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, id: Path<i64>, section_id: Path<i64>, recipe_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        let owns_cookbook = permission::cookbook::owns_cookbook(pool.0, id.0, auth);
        let owns_recipe = permission::recipe::owns_recipe(pool.0, recipe_id.0, auth);
        try_join!(owns_cookbook, owns_recipe)?;
        cookbook::remove_recipe(pool.0, storage.0, section_id.0, recipe_id.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/section", method = "post")]
    async fn add_section(&self, pool: Data<&MySqlPool>, id: Path<i64>, section: Json<cookbook::Section>, auth: JWTAuthorization) -> Result<()> {
        permission::cookbook::owns_cookbook(pool.0, id.0, auth).await?;
        let sections = cookbook::get_sections(pool.0, id.0).await?;
        cookbook::add_section(pool.0, id.0, section.0, sections.len().try_into().unwrap()).await?;
        Ok(())
    }

    #[oai(path = "/:id/section/:section_id", method = "delete")]
    async fn remove_section(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, id: Path<i64>, section_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::cookbook::owns_cookbook(pool.0, id.0, auth).await?;
        cookbook::remove_section(pool.0, storage.0, section_id.0, id.0).await?;
        Ok(())
    }

    #[oai(path = "/:id/pages", method = "get")]
    async fn get_pages(&self, pool: Data<&MySqlPool>, id: Path<i64>, auth: JWTAuthorization) -> Result<GetCookbookPagesResponse> {
        permission::cookbook::is_visible(pool.0, id.0, auth).await?;
        let mut pages = Vec::new();
        let sections = cookbook::get_sections(pool.0, id.0).await?;
        for section in sections {
            let section_id = section.id;
            let recipes = recipe::get_cookbook_section_recipes(pool.0, section.id, auth.0).await?;
            let section_page = page::Page::Section(section);
            pages.push(section_page);
            for recipe in recipes {
                let recipe_id = recipe.id;
                let recipe_page = page::Page::Recipe(recipe);
                pages.push(recipe_page);
                if cookbook::recipe_has_pic(pool.0, section_id, recipe_id).await? {
                    let image = format!("cookbook/{}/section/{}/recipe/{}/image", id.0, section_id, recipe_id);
                    let image_page = page::Page::Image(page::ImagePage { image });
                    pages.push(image_page);
                }
            }
        }
        Ok(Json(pages))
    }

    #[oai(path = "/:id/section/:section_id/recipe/:recipe_id/image", method = "put")]
    async fn set_recipe_pic(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, id: Path<i64>, section_id: Path<i64>, recipe_id: Path<i64>, pic: cookbook::SetRecipePic, auth: JWTAuthorization) -> Result<()> {
        permission::cookbook::owns_cookbook(pool.0, id.0, auth).await?;
        cookbook::set_recipe_pic(pool.0, storage.0, id.0, section_id.0, recipe_id.0, pic).await?;
        Ok(())
    }

    #[oai(path = "/:id/section/:section_id/recipe/:recipe_id/image", method = "get")]
    async fn get_recipe_pic(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, id: Path<i64>, section_id: Path<i64>, recipe_id: Path<i64>, auth: JWTAuthorization) -> Result<GetRecipePicResponse> {
        permission::cookbook::is_visible(pool.0, id.0, auth).await?;
        let result = cookbook::get_recipe_pic(pool.0, storage.0, section_id.0, recipe_id.0).await?;
        Ok(
            match result {
                cookbook::RecipePicResult::PicNotFound => {
                    GetRecipePicResponse::NotFound(PlainText("Image not found".to_string()))
                },
                cookbook::RecipePicResult::RecipeNotFound => {
                    GetRecipePicResponse::NotFound(PlainText("Recipe not found".to_string()))
                },
                cookbook::RecipePicResult::Ok(attachment) => {
                    GetRecipePicResponse::Ok(attachment)
                }
            }
        )
    }

    #[oai(path = "/:id/section/:section_id/recipe/:recipe_id/image", method = "delete")]
    async fn remove_recipe_pic(&self, pool: Data<&MySqlPool>, storage: Data<&DufsStorage>, id: Path<i64>, section_id: Path<i64>, recipe_id: Path<i64>, auth: JWTAuthorization) -> Result<()> {
        permission::cookbook::owns_cookbook(pool.0, id.0, auth).await?;
        cookbook::remove_recipe_pic(pool.0, storage.0, section_id.0, recipe_id.0).await?;
        Ok(())
    }
}
