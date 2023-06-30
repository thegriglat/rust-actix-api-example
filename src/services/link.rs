use crate::db::schema::urls::dsl::*;
use crate::{models::url::Url, Pool};
use actix_web::web;
use anyhow::{anyhow, Result};
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::Error::NotFound;
use diesel::QueryDsl;

pub fn find_one(shorten_url: &str, pool: web::Data<Pool>) -> Result<Url> {
    let conn = &mut pool.get()?;
    urls.find(shorten_url)
        .first::<Url>(conn)
        .map_err(|err| match err {
            NotFound => anyhow!("Url not found"),
            err => anyhow!(err),
        })
}

pub fn create(full_url: &str, pool: web::Data<Pool>) -> Result<Url> {
    let conn = &mut pool.get()?;

    let shorten_url = shortify(full_url);

    let db_row = find_one(&shorten_url, pool);
    if db_row.is_ok() {
        return Ok(Url {
            url: full_url.to_owned(),
            short_url: shorten_url,
        });
    }

    let new_url = Url {
        url: full_url.to_owned(),
        short_url: shortify(full_url),
    };

    insert_into(urls)
        .values(&new_url)
        .execute(conn)
        .map(|_| new_url)
        .map_err(|err| anyhow!(err))
}

fn shortify(str: &str) -> String {
    // 128 bits = 16 u8
    let digest: Vec<u8> = md5::compute(str).iter().copied().take(4).collect();
    bs58::encode(digest).into_string()
}

#[cfg(test)]
mod tests {
    use super::shortify;

    #[test]
    fn shortify_symbols() {
        let input = "https://google.com";
        let shortified = shortify(input);
        assert_eq!(shortified.len(), 6);
    }
}
