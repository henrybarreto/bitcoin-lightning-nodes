use actix_web::{
    body::BoxBody, get, http::header::ContentType, web, HttpRequest, HttpResponse, Responder,
};

use crate::database;

impl Responder for database::Node {
    // TODO: What is a BoxBody?
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/nodes")]
async fn get_nodes(data: web::Data<database::Database>) -> web::Json<Vec<database::Node>> {
    return match data.get_ref().get_nodes().await {
        Ok(results) => web::Json(results),
        Err(_) => web::Json(Vec::new()),
    };
}

#[get("/healthcheck")]
async fn healthcheck() -> web::Json<()> {
    return web::Json(());
}
