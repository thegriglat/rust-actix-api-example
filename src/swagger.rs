use crate::api::dto::{LinkDto, UrlRequest};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(crate::api::links::get_link, crate::api::links::post_link),
    components(schemas(LinkDto, UrlRequest))
)]
pub struct ApiDoc;
