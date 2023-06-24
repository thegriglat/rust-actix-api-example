use actix_web::{web, App, HttpServer};
use config::Config;
use dotenv::dotenv;
use state::AppState;

mod api;
mod config;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::read();

    let app_data = web::Data::new(AppState::new());
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(api::links::get_link)
            .service(api::links::post_link)
    })
    .bind(("127.0.0.1", config.port))?
    .workers(config.workers)
    .run()
    .await
}
