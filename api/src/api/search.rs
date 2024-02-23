use poem_openapi::{OpenApi, payload::Json, param::Path, Tags};
use poem::{web::Data, Result};
use sqlx::MySqlPool;
use crate::api::auth::JWTAuthorization;
use crate::model::{post, recipe, community, tag, user};

#[derive(Tags)]
enum ApiTags {
    Search
}

// Responses

type SearchPost = Json<Vec<post::PostResult>>;

type SearchRecipe = Json<Vec<recipe::RecipeResult>>;

type SearchCommunity = Json<Vec<community::CommunityResult>>;

type SearchTag = Json<Vec<tag::TagResult>>;

type SearchUser = Json<Vec<user::FollowResult>>;

pub struct SearchApi;

#[OpenApi(prefix_path = "/search", tag = "ApiTags::Search")]
impl SearchApi {

    #[oai(path = "/post/:search", method = "get")]
    async fn search_post(&self, pool: Data<&MySqlPool>, search: Path<String>, auth: JWTAuthorization) -> Result<SearchPost> {
        let posts = post::search_posts(pool.0, search.0, auth.0).await?;
        Ok(Json(posts))
    }

    #[oai(path = "/recipe/:search", method = "get")]
    async fn search_recipe(&self, pool: Data<&MySqlPool>, search: Path<String>, auth: JWTAuthorization) -> Result<SearchRecipe> {
        let recipes = recipe::search_recipes(pool.0, search.0, auth.0).await?;
        Ok(Json(recipes))
    }

    #[oai(path = "/community/:search", method = "get")]
    async fn search_community(&self, pool: Data<&MySqlPool>, search: Path<String>, auth: JWTAuthorization) -> Result<SearchCommunity> {
        let communities = community::search_communities(pool.0, search.0, auth.0).await?;
        Ok(Json(communities))
    }

    #[oai(path = "/tag/:search", method = "get")]
    async fn search_tag(&self, pool: Data<&MySqlPool>, search: Path<String>, auth: JWTAuthorization) -> Result<SearchTag> {
        let tags = tag::search_tags(pool.0, search.0, auth.0).await?;
        Ok(Json(tags))
    }

    #[oai(path = "/user/:search", method = "get")]
    async fn search_user(&self, pool: Data<&MySqlPool>, search: Path<String>) -> Result<SearchUser> {
        let users = user::search_users(pool.0, search.0).await?;
        Ok(Json(users))
    }
}
