use actix_web::{App, HttpServer};
use config::Config;
use dotenv::dotenv;

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::read();
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", config.port))?
        .run()
        .await
}
