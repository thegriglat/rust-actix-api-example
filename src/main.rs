use actix_web::{web, App, HttpServer};
use config::Config;
use dotenv::dotenv;

mod config;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::read();

    HttpServer::new(|| {
        App::new().service(
            web::scope("api")
                .service(web::scope("auth").service(routes::auth::authorization::get_jwt_token)),
        )
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
