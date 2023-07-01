use super::dto::*;
use crate::errors::AppError;
use crate::services::link::{create, find_one};
use crate::Pool;
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_validator::Json;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("links").service(get_link).service(post_link));
}

#[utoipa::path(
    get,
    path = "/api/links/{short_link}",
    responses(
        (status = 200, description = "returns full link", body = LinkDto),
        (status = NOT_FOUND, description = "Link not found")
    ),
    params(
        ("short_link" = String, Path, description = "short link"),
    )
)]
#[get("{short_link}")]
pub async fn get_link<'a>(path: web::Path<String>, pool: web::Data<Pool>) -> impl Responder {
    let short_link = path.into_inner();
    let db_link = find_one(&short_link, pool);
    match db_link {
        Ok(row) => HttpResponse::Ok().json(LinkDto::new(row.url)),
        Err(err) => HttpResponse::NotFound().json(AppError::from(err)),
    }
}

#[utoipa::path(
    post,
    path = "/api/links",
    responses(
        (status = 201, description = "returns short link", body = LinkDto),
        (status = NOT_FOUND, description = "Error")
    ),
    request_body = UrlRequest
)]
#[post("")]
pub async fn post_link<'a>(pool: web::Data<Pool>, json: Json<UrlRequest>) -> impl Responder {
    let request_url = json.url.clone();
    let db_link = create(&request_url, pool);
    match db_link {
        Ok(row) => HttpResponse::Created().json(LinkDto::new(row.short_url)),
        Err(err) => HttpResponse::InternalServerError().json(AppError::from(err)),
    }
}
