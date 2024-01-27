use std::env;
use poem::{listener::TcpListener, Route, EndpointExt, middleware::Cors};
use poem_openapi::OpenApiService;
use cookbook::api;
use cookbook::storage::dufs::DufsStorage;
use sqlx::MySqlPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL")?;
    let pool = MySqlPool::connect(&db_url).await?;

    let storage_url = env::var("STORAGE_URL")?;
    let storage = DufsStorage::new(&storage_url);

    let jwt_secret = env::var("SECRET")?;

    let production = env::var("PRODUCTION").is_ok();

    let apis = (
        api::user::UserApi, api::auth::AuthApi, api::post::PostApi, api::recipe::RecipeApi,
        api::community::CommunityApi
    );

    let api_service = 
        OpenApiService::new(apis, "CookBook API", "1.0").server("http://localhost:8000/api");
   
    let app = 
        if production {
            Route::new()
        }
        else {
            let ui = api_service.swagger_ui();
            Route::new().nest("/", ui)
        }
        .nest("/api", api_service)
        .with(Cors::new().allow_credentials(true))
        .data(pool)
        .data(storage)
        .data(jwt_secret);

    Ok(poem::Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?)
}
