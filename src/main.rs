mod distribution;
use distribution::crc32_num;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    entity_id: String,
    salt: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Flagge.rs!")
}

#[get("/evaluate")]
async fn evaluate(info: web::Query<Info>) -> String {
    crc32_num(&info.entity_id, &info.salt).to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
            App::new()
            .service(hello)
            .service(evaluate)
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
