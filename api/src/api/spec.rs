use poem_openapi::{OpenApi, payload::PlainText};
use poem::web::Data;

#[derive(Clone)]
pub struct Spec(pub String);

pub struct SpecApi;

#[OpenApi]
impl SpecApi {

    #[oai(path = "/openapi.json", method = "get")]
    async fn get_spec(&self, spec: Data<&Spec>) -> PlainText<String> {
        let spec = String::from(&spec.0.0);
        PlainText(spec)
    }

}
