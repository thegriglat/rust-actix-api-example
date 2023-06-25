use std::collections::HashMap;

use actix_web::{error, middleware::Logger, web, App, HttpResponse, HttpServer};
use actix_web_validator::JsonConfig;
use config::Config;
use diesel::{
    r2d2::{self, ConnectionManager},
    SqliteConnection,
};
use errors::ValidationErrorResponse;

mod api;
mod config;
mod db;
mod errors;
mod models;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::read();

    let manager = ConnectionManager::<SqliteConnection>::new(config.database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // db pool app data
    let app_data = web::Data::new(pool.clone());

    // server
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .app_data(JsonConfig::default().error_handler(|err, _| {
                let json_error = match &err {
                    actix_web_validator::Error::Validate(error) => {
                        ValidationErrorResponse::from(error)
                    }
                    _ => ValidationErrorResponse {
                        messages: HashMap::new(),
                    },
                };
                error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(json_error),
                )
                .into()
            }))
            .service(api::links::get_link)
            .service(api::links::post_link)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", config.port))?
    .workers(config.workers)
    .run()
    .await
}
