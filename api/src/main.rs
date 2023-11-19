use std::env;
use poem::{listener::TcpListener, Route, EndpointExt};
use poem_openapi::OpenApiService;
use api::api;
use sqlx::MySqlPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL")?;
    let pool = MySqlPool::connect(&db_url).await?;

    let api_service = 
        OpenApiService::new((api::user::UserApi, api::auth::AuthApi), "CookBook API", "1.0").server("http://localhost:8000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/", ui)
        .data(pool);

    Ok(poem::Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?)
}