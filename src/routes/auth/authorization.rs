use actix_web::{get, HttpResponse, Responder};

use super::dto;

#[get("login")]
pub async fn get_jwt_token() -> impl Responder {
    let token = "my_awesome_token".to_string();
    HttpResponse::Ok().json(dto::JwtTokenResponse::new(token))
}
