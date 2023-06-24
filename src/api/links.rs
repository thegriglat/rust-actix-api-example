use super::dto::*;
use crate::state::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/{short_link}")]
pub async fn get_link(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let short_link = path.into_inner();
    match data.get(&short_link) {
        Some(full_link) => HttpResponse::Ok().json(ShortenedLink::new(full_link)),
        None => HttpResponse::NotFound().body(""),
    }
}

#[post("/")]
pub async fn post_link(data: web::Data<AppState>, json: web::Json<UrlRequest>) -> impl Responder {
    let url = json.url.clone();
    let short_url = data.add(url);
    HttpResponse::Ok().json(UrlRequest::new(short_url))
}
