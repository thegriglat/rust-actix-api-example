use actix_web::{middleware::Logger, web, App, HttpServer};
use config::Config;
use diesel::{
    r2d2::{self, ConnectionManager},
    SqliteConnection,
};
use dotenv::dotenv;

mod api;
mod config;
mod db;
mod errors;
mod models;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::read();

    let manager = ConnectionManager::<SqliteConnection>::new(config.database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_data = web::Data::new(pool.clone());
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(api::links::get_link)
            .service(api::links::post_link)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", config.port))?
    .workers(config.workers)
    .run()
    .await
}
