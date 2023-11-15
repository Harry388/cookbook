use poem_openapi::{payload::PlainText, OpenApi};
use poem::web::Data;
use sqlx::MySqlPool;

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/test", method = "get")]
    async fn index(&self, pool: Data<&MySqlPool>) -> PlainText<String> {
        let results = sqlx::query!(
                "select * from test"
            )
            .fetch_all(pool.0)
            .await;
        PlainText("Success".to_string())
    }
}