use crate::{models::url_model::Url, repository::mongodb_repo::MongoRepo};
use actix_web::{
    get,
    post,
    web::{Data, Json, Path},
    http::header::{LOCATION, HeaderValue},
    HttpResponse,
};

#[get("/{short_url}")]
pub async fn get_url(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let short_url = path.into_inner();

    if short_url.is_empty() {
        return HttpResponse::NotFound().finish();
    }

    let url_detail = db.get_url(&short_url).await;

    match url_detail {
        Ok(url) => {
            let header = HeaderValue::from_str(&url.destination)
                .unwrap();

            return HttpResponse::MovedPermanently()
                .append_header((LOCATION, header))
                .finish()
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/url")]
pub async fn create_url(db: Data<MongoRepo>, new_url: Json<Url>) -> HttpResponse {
    let data = Url {
        id: None,
        short_url: new_url.short_url.to_owned(),
        destination: new_url.destination.to_owned(),
    };

    let url_detail = db.create_url(data).await;

    match url_detail {
        Ok(url) => HttpResponse::Ok().json(url),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
