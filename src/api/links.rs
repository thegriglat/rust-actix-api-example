use self::db::schema::urls::dsl::*;
use super::dto::*;
use crate::db;
use crate::errors::AppError;
use crate::models::url::Url;
use crate::Pool;
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_validator::Json;
use anyhow::{anyhow, Result};
use bs58;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::Error::NotFound;
use diesel::QueryDsl;

#[get("/{short_link}")]
pub async fn get_link<'a>(path: web::Path<String>, pool: web::Data<Pool>) -> impl Responder {
    let short_link = path.into_inner();
    let db_link = get_link_db(&short_link, pool);
    match db_link {
        Ok(row) => HttpResponse::Ok().json(LinkDto::new(row.url)),
        Err(err) => HttpResponse::NotFound().json(AppError::from(err)),
    }
}

#[post("/")]
pub async fn post_link<'a>(pool: web::Data<Pool>, json: Json<UrlRequest>) -> impl Responder {
    let request_url = json.url.clone();
    let db_link = add_link_db(&request_url, pool);
    match db_link {
        Ok(row) => HttpResponse::Ok().json(LinkDto::new(row.short_url)),
        Err(err) => HttpResponse::InternalServerError().json(AppError::from(err)),
    }
}

fn get_link_db(shorten_url: &str, pool: web::Data<Pool>) -> Result<Url> {
    let conn = &mut pool.get()?;
    urls.find(shorten_url)
        .first::<Url>(conn)
        .map_err(|err| match err {
            NotFound => anyhow!("Url not found"),
            err => anyhow!(err),
        })
}

fn add_link_db(full_url: &str, pool: web::Data<Pool>) -> Result<Url> {
    let conn = &mut pool.get()?;

    let shorten_url = get_short_link(full_url);

    let db_row = get_link_db(&shorten_url, pool);
    if db_row.is_ok() {
        return Ok(Url {
            url: full_url.to_owned(),
            short_url: shorten_url,
        });
    }

    let new_url = Url {
        url: full_url.to_owned(),
        short_url: get_short_link(full_url),
    };

    insert_into(urls)
        .values(&new_url)
        .execute(conn)
        .map(|_| new_url)
        .map_err(|err| anyhow!(err))
}

fn get_short_link(str: &str) -> String {
    // 128 bits = 16 u8
    let digest: Vec<u8> = md5::compute(str).to_vec().into_iter().take(4).collect();
    bs58::encode(digest).into_string()
}
