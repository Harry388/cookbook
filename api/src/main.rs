use std::env;
use poem::{listener::TcpListener, Route, EndpointExt, middleware::Cors};
use poem_openapi::OpenApiService;
use cookbook::api;
use sqlx::MySqlPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL")?;
    let pool = MySqlPool::connect(&db_url).await?;

    let apis = (
        api::user::UserApi, api::auth::AuthApi, api::post::PostApi
    );

    let api_service = 
        OpenApiService::new(apis, "CookBook API", "1.0").server("http://localhost:8000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/", ui)
        .with(Cors::new().allow_credentials(true))
        .data(pool);

    Ok(poem::Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?)
}