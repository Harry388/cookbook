use std::{env, fs};
use poem::{listener::{Listener, RustlsCertificate, RustlsConfig, TcpListener}, middleware::Cors, EndpointExt, Route};
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
        api::community::CommunityApi, api::comment::CommentApi, api::album::AlbumApi, api::tag::TagApi, api::search::SearchApi,
        api::cookbook::CookbookApi, api::spec::SpecApi
    );

    let api_service = 
        OpenApiService::new(apis, "CookBook API", "1.0").server("http://localhost:8000/api");

    let spec = api::spec::Spec(api_service.spec());

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
        .data(jwt_secret)
        .data(spec);

    let listener = TcpListener::bind("0.0.0.0:8000");

    if production {
        let key = fs::read("/cert/key.pem")?;
        let cert = fs::read("/cert/cert.pem")?;

        let listener = listener.rustls(RustlsConfig::new().fallback(RustlsCertificate::new().key(key).cert(cert)));

        Ok(poem::Server::new(listener)
            .run(app)
            .await?)
    }
    else {
        Ok(poem::Server::new(listener)
            .run(app)
            .await?)
    }

}
