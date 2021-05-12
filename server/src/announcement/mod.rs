//! This file contains the code for the `/announcement/*` endpoints.
use crate::{actions, model, Authorized, DbPool};
use actix_web::{web, HttpResponse};
use serde_json::json;

mod request;
mod response;

/// Adds the endpoints under `/announcement/*` to the supplied App object and
/// returns the modified App.
pub fn add_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.route("/announcement/new", web::post().to(new))
        .route("/announcement/find/{id}", web::get().to(find))
        .route("/announcement/list", web::get().to(list));
}

/// Handles `/announcement/new`
async fn new(
    authorized: Authorized,
    req: web::Json<request::NewAnnouncement>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, HttpResponse> {
    use actix_web::error::BlockingError;

    let req = req.into_inner();

    // Only admins can create new announcements
    if !authorized.user.is_admin {
        return Err(HttpResponse::Forbidden().into());
    }

    let image_id = req.image.clone();
    let conn = pool.get().expect("Couldn't connect to the database");
    match web::block(move || actions::get_image_by_id(&conn, &image_id)).await {
        Ok(metadata) if !metadata.private => {}
        Err(BlockingError::Canceled) => return Err(HttpResponse::InternalServerError().into()),
        // Either ok with private image or the diesel request failed (probably becaue there as no
        // image)
        _ => {
            return Err(HttpResponse::BadRequest().json(json!({
                "error": "The provided image hasn't been uploaded yet or isn't public",
                "error_code": "MISSING_OR_HIDDEN_IMAGE"
            })))
        }
    };

    let new_announcement = model::NewAnnouncement {
        title: req.title,
        subtitle: req.subtitle,
        image: req.image,
    };

    let announcement = match web::block(move || {
        let conn = pool.get().expect("Couldn't connect to the database");
        actions::new_announcement(&conn, new_announcement)
    })
    .await
    {
        Ok(announcement) => announcement,
        Err(_) => return Err(HttpResponse::InternalServerError().into()),
    };

    Ok(HttpResponse::Ok().json(json!({ "id": announcement.id })))
}

/// Handles `/announcement/find/{ID}`
async fn find(path: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, HttpResponse> {
    let id = path.into_inner();

    let announcement = match web::block(move || {
        let conn = pool.get().expect("Couldn't connect to the database");
        actions::get_announcement_by_id(&conn, &id)
    })
    .await
    {
        Ok(announcement) => announcement,
        Err(_) => return Err(HttpResponse::NotFound().into()),
    };

    Ok(HttpResponse::Ok().json(response::AnnouncementResponse::from(announcement)))
}

/// Handles `/announcement/list`
async fn list(pool: web::Data<DbPool>) -> Result<HttpResponse, HttpResponse> {
    let announcements = match web::block(move || {
        let conn = pool.get().expect("Couldn't connect to the database");
        actions::list_visible_announcements(&conn)
    })
    .await
    {
        Ok(announcements) => announcements,
        Err(_) => return Err(HttpResponse::NotFound().into()),
    };

    Ok(HttpResponse::Ok().json(response::AnnouncementList::from(announcements)))
}
