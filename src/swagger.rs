use crate::api::dto::{LinkDto, UrlRequest};
use actix_web::web::ServiceConfig;
use utoipa::OpenApi;

#[cfg(feature = "swagger")]
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(crate::api::links::get_link, crate::api::links::post_link),
    components(schemas(LinkDto, UrlRequest))
)]
pub struct ApiDoc;

pub fn configure(_cfg: &mut ServiceConfig) {
    cfg_if::cfg_if! {
        if #[cfg(feature = "swagger")] {
            _cfg.service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            );
        }
    }
}

pub fn swagger_status() {
    cfg_if::cfg_if! {
        if #[cfg(feature = "swagger")] {
            println!("Swagger enabled")
        } else {
            println!("Swagger disabled")
        }
    }
}
