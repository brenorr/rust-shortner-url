mod api;
mod models;
mod repository;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use api::url_api::{create_url, get_url};
use repository::mongodb_repo::MongoRepo;

#[get("/_health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("App Health")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_url)
            .service(health)
            .service(get_url)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
