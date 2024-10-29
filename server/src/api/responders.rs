#![allow(dead_code)]

use rocket::Responder;

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct Success(pub String);

#[derive(Responder)]
pub struct AlreadyExist(pub String);

#[derive(Responder)]
#[response(status = 400, content_type = "json")]
pub struct InternalError(pub String);

#[derive(Responder)]
pub enum Error {
    #[response(status = 400, content_type = "json")]
    Unauthorized(String),
    #[response(status = 500, content_type = "json")]
    #[allow(clippy::enum_variant_names)]
    InternalError(String),
    #[response(status = 409, content_type = "json")]
    Conflict(String),
    #[response(status = 404, content_type = "json")]
    NotFound(String),
}
