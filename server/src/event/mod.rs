//! This file contains the code for the `/event/*` endpoints.
use crate::{actions, model, Authorized, DbPool};
use actix_web::{web, HttpResponse};
use serde_json::json;

mod request;
mod response;

/// Represents a single line of a lineup.
/// This is shared between both request and response.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LineupItem {
    pub label: String,
    pub description: String,
}

impl From<model::LineupItem> for LineupItem {
    fn from(l: model::LineupItem) -> Self {
        LineupItem {
            label: l.0,
            description: l.1,
        }
    }
}

impl From<LineupItem> for model::LineupItem {
    fn from(l: LineupItem) -> Self {
        model::LineupItem(l.label, l.description)
    }
}

/// Adds the endpoints under `/event/*` to the supplied App object and
/// returns the modified App.
pub fn add_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.route("/event/new", web::post().to(new))
        .route("/event/find/{id}", web::get().to(find))
        .route("/event/list", web::get().to(list));
}

/// Handles `/event/new`
async fn new(
    authorized: Authorized,
    req: web::Json<request::NewEvent>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, HttpResponse> {
    let req = req.into_inner();

    // Only admins can create new events
    if !authorized.user.is_admin {
        return Err(HttpResponse::Forbidden().into());
    }

    match web::block({
        let pool = pool.clone();
        let image_ids = req.images.clone();
        move || {
            let conn = pool.get().expect("Couldn't connect to the database");
            actions::get_images_by_ids(&conn, image_ids)
        }
    })
    .await
    {
        Ok(images) => {
            if images.len() != req.images.len() {
                return Err(HttpResponse::BadRequest().json(json!({
                    "error": "The provided image hasn't been uploaded yet or isn't public",
                    "error_code": "MISSING_OR_HIDDEN_IMAGE"
                })));
            }
            for image in &images {
                if image.private {
                    return Err(HttpResponse::BadRequest().json(json!({
                        "error": "The provided image hasn't been uploaded yet or isn't public",
                        "error_code": "MISSING_OR_HIDDEN_IMAGE"
                    })));
                }
            }
        }
        Err(_) => return Err(HttpResponse::InternalServerError().into()),
    };

    let new_event = model::NewEvent {
        pre_title: req.pre_title,
        title: req.title,
        description: req.description,
        location: req.location,
        is_virtual: req.is_virtual,
        virtual_link: req.virtual_link,
        lineup: req.lineup.into_iter().map(|item| item.into()).collect(),
        price_pence: req.price_pence,
        images: req.images,
        visible: req.visible,
        event_start: crate::util::epoch_seconds_to_pgtimestamp(req.event_start),
    };

    let event = match web::block(move || {
        let conn = pool.get().expect("Couldn't connect to the database");
        actions::new_event(&conn, new_event)
    })
    .await
    {
        Ok(event) => event,
        Err(_) => return Err(HttpResponse::InternalServerError().into()),
    };

    Ok(HttpResponse::Ok().json(json!({ "id": event.id })))
}

/// Handles `/event/find/{ID}`
async fn find(path: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, HttpResponse> {
    let id = path.into_inner();

    let event = match web::block(move || {
        let conn = pool.get().expect("Couldn't connect to the database");
        actions::get_event_by_id(&conn, &id)
    })
    .await
    {
        Ok(event) => event,
        Err(_) => return Err(HttpResponse::NotFound().into()),
    };

    Ok(HttpResponse::Ok().json(response::EventResponse::from(event)))
}

/// Handles `/event/list`
async fn list(pool: web::Data<DbPool>) -> Result<HttpResponse, HttpResponse> {
    let events = match web::block(move || {
        let conn = pool.get().expect("Couldn't connect to the database");
        actions::list_visible_events(&conn)
    })
    .await
    {
        Ok(events) => events,
        Err(_) => return Err(HttpResponse::NotFound().into()),
    };

    Ok(HttpResponse::Ok().json(response::EventList::from(events)))
}
