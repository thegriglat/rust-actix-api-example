use std::{collections::HashMap, net::Ipv4Addr};

use actix_web::{error, middleware::Logger, web, App, HttpResponse, HttpServer};
use actix_web_validator::JsonConfig;
use config::Config;
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use errors::ValidationErrorResponse;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod config;
mod db;
mod errors;
mod models;
mod services;
mod swagger;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::read();

    let manager = ConnectionManager::<PgConnection>::new(config.database_url);
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
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", swagger::ApiDoc::openapi()),
            )
            .service(web::scope("api").configure(api::links::configure))
            .wrap(Logger::default())
    })
    .bind((Ipv4Addr::UNSPECIFIED, config.port))
    .unwrap()
    .workers(config.workers)
    .run()
    .await
}
