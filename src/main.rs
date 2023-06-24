use std::env;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::vars()
        .find(|x| x.0 == "PORT")
        .expect("Specify PORT environment variable")
        .1
        .parse::<u16>()
        .expect("Port is not a number");

    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
