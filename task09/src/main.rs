use actix_web::{web, App};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct IndexParams {
    username: String,
}

#[derive(Serialize)]
struct IndexResponse {
    message: String,
}

async fn index(_: web::HttpRequest, p: web::Query<IndexParams>) -> impl web::Responder {
    web::HttpResponse::Ok().json(IndexResponse {
        message: format!("Hello, {}", p.username),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
