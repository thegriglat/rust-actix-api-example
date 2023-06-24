use actix_web::{middleware::Logger, web, App, HttpServer};
use config::Config;
use dotenv::dotenv;
use state::AppState;

mod api;
mod config;
mod errors;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::read();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_data = web::Data::new(AppState::new());
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
