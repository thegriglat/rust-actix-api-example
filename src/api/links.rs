use self::db::schema::urls::dsl::*;
use super::dto::*;
use crate::db;
use crate::errors::AppError;
use crate::models::url::Url;
use crate::Pool;
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_validator::Json;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::QueryDsl;
use uuid::Uuid;

#[get("/{short_link}")]
pub async fn get_link<'a>(path: web::Path<String>, pool: web::Data<Pool>) -> impl Responder {
    let short_link = path.into_inner();
    let db_link = get_link_db(&short_link, pool);
    match db_link {
        Ok(row) => HttpResponse::Ok().json(LinkDto::new(row.url)),
        Err(_) => HttpResponse::NotFound().json(AppError::new("Url not found")),
    }
}

#[post("/")]
pub async fn post_link<'a>(pool: web::Data<Pool>, json: Json<UrlRequest>) -> impl Responder {
    let request_url = json.url.clone();
    let db_link = add_link_db(&request_url, pool);
    match db_link {
        Ok(row) => HttpResponse::Ok().json(LinkDto::new(row.short_url)),
        Err(_) => HttpResponse::InternalServerError().json(AppError::new("Db error")),
    }
}

fn get_link_db(shorten_url: &str, pool: web::Data<Pool>) -> Result<Url, diesel::result::Error> {
    let conn = &mut pool.get().unwrap();
    urls.find(shorten_url).first::<Url>(conn)
}

fn add_link_db(full_url: &str, pool: web::Data<Pool>) -> Result<Url, diesel::result::Error> {
    let conn = &mut pool.get().unwrap();
    let new_url = Url {
        url: full_url.to_owned(),
        short_url: Uuid::new_v4().to_string(),
    };

    insert_into(urls)
        .values(&new_url)
        .execute(conn)
        .map(|_| new_url)
}
