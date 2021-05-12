//! This file contains the code for the `/user/*` endpoints.
use crate::Authorized;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

mod request;
mod response;

/// Adds the endpoints under `/user/*` to the supplied App object and
/// returns the modified App.
pub fn add_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.route("/user/info", web::post().to(info));
}

/// Handles `/user/info`
async fn info(authorized: Authorized) -> impl Responder {
    HttpResponse::Ok().json(json!({ "user": response::UserResponse::from(authorized.user) }))
}
