use super::dto::*;
use crate::errors::AppError;
use crate::services::link::{create, find_one};
use crate::Pool;
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_validator::Json;

#[get("/{short_link}")]
pub async fn get_link<'a>(path: web::Path<String>, pool: web::Data<Pool>) -> impl Responder {
    let short_link = path.into_inner();
    let db_link = find_one(&short_link, pool);
    match db_link {
        Ok(row) => HttpResponse::Ok().json(LinkDto::new(row.url)),
        Err(err) => HttpResponse::NotFound().json(AppError::from(err)),
    }
}

#[post("/")]
pub async fn post_link<'a>(pool: web::Data<Pool>, json: Json<UrlRequest>) -> impl Responder {
    let request_url = json.url.clone();
    let db_link = create(&request_url, pool);
    match db_link {
        Ok(row) => HttpResponse::Ok().json(LinkDto::new(row.short_url)),
        Err(err) => HttpResponse::InternalServerError().json(AppError::from(err)),
    }
}
