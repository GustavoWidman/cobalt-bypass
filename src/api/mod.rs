use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    url: String,
}

#[post("/")]
async fn get(info: web::Json<Info>) -> impl Responder {
    match crate::utils::cobalt(info.url.to_string()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}
