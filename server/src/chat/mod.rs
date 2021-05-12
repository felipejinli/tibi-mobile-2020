//! The file contains the endpoints for `/chat/*`

use crate::{
    actions, error::ServerErrorTypes, model, try_or_log_error, Authorized, DbPool, RedisPool,
};
use actix_web::{web, HttpRequest, HttpResponse};
use slog::{error, warn, Logger};

use serde_json::json;

mod live;

mod request;
mod response;

pub const REDIS_CHAT_PREFIX: &'static str = "CHAT_";

/// Adds the endpoints under `/chat/*` to the supplied App object and
/// returns the modified App.
pub fn add_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.route("/chat/connect", web::get().to(connect))
        .route("/chat/create_dm", web::post().to(create_dm))
        .route("/chat/room_info", web::post().to(room_info))
        .route("/chat/retrieve", web::post().to(retrieve));
}

/// Handles `/chat/connect`
async fn connect(
    req: HttpRequest,
    stream: web::Payload,
    root_logger: web::Data<Logger>,
    redis_pool: web::Data<RedisPool>,
    pg_pool: web::Data<DbPool>,
    config: web::Data<crate::Config>,
) -> Result<HttpResponse, HttpResponse> {
    let logger = root_logger.new(
        slog::o!("endpoint" =>  "/chat/connect", "host" => req.connection_info().host().to_owned()),
    );

    actix_web_actors::ws::start(
        live::LiveChatHandler::new(
            logger.clone(),
            pg_pool.into_inner(),
            redis_pool.into_inner(),
            config.jwt_secret.clone(),
        )
        .await?,
        &req,
        stream,
    )
    .map_err(|e| {
        error!(logger, "couldn't start live chat handler: {}", e);
        HttpResponse::InternalServerError().json(json!({
            "error": "Internal server error",
            "error_code": "INTERNAL_SERVER_ERROR"
        }))
    })
}

/// Handles `/chat/retrieve`
async fn retrieve(
    authorized: Authorized,
    req: HttpRequest,
    request: web::Json<request::Retrieve>,
    db_pool: web::Data<DbPool>,
    root_logger: web::Data<Logger>,
) -> Result<HttpResponse, HttpResponse> {
    let logger = root_logger.new(
        slog::o!("endpoint" =>  "/chat/retrieve", "host" => req.connection_info().host().to_owned(), "user_id" => authorized.user.id.clone()),
    );

    let messages = try_or_log_error!(
        logger,
        web::block(move || {
            let pg_conn = db_pool.get().map_err(|e| ServerErrorTypes::from(e))?;
            actions::get_user_messages_after(&pg_conn, &authorized.user.id, request.after)
                .map_err(|e| ServerErrorTypes::from(e))
        })
        .await
    );

    Ok(HttpResponse::Ok().json(response::Retrieve::from(messages)))
}

/// Handles `/chat/create_dm`
async fn create_dm(
    authorized: Authorized,
    req: HttpRequest,
    request: web::Json<request::CreateDM>,
    db_pool: web::Data<DbPool>,
    root_logger: web::Data<Logger>,
) -> Result<HttpResponse, HttpResponse> {
    // TODO: Check if dm already exists (will require marking certain rooms as being DM rooms)
    let logger = root_logger.new(
        slog::o!("endpoint" =>  "/chat/create_dm", "host" => req.connection_info().host().to_owned(), "user_id" => authorized.user.id.clone()),
    );
    let other_id = request.into_inner().other;

    let other = try_or_log_error!(
        logger,
        web::block({
            let db_pool = db_pool.clone();
            let other_id = other_id.clone();
            move || {
                let pg_conn = db_pool.get().map_err(|e| ServerErrorTypes::from(e))?;
                crate::actions::try_get_user_by_id(&pg_conn, &other_id)
                    .map_err(|e| ServerErrorTypes::from(e))
            }
        })
        .await
    );

    let other = other.ok_or_else(|| {
        warn!(logger, "other user `{}` didn't exist", other_id);
        HttpResponse::NotFound().json(serde_json::json!({
            "error": "The user you tried to create a DM with doesn't exist",
            "error_code": "USER_NOT_FOUND",
        }))
    })?;

    let room = try_or_log_error!(
        logger,
        web::block({
            let db_pool = db_pool.clone();
            move || {
                let pg_conn = db_pool.get().map_err(|e| ServerErrorTypes::from(e))?;

                let new_chat_room = model::NewChatRoom { name: None };

                crate::actions::new_chat_room(&pg_conn, new_chat_room)
                    .map_err(|e| ServerErrorTypes::from(e))
            }
        })
        .await
    );

    try_or_log_error!(
        logger,
        web::block({
            let db_pool = db_pool.clone();
            move || {
                let pg_conn = db_pool.get().map_err(|e| ServerErrorTypes::from(e))?;

                let current_user = model::NewRoomOccupant {
                    room_id: room.id,
                    user_id: authorized.user.id,
                    can_send: true,
                    can_add_user: false,
                    can_change_name: false,
                };
                let other_user = model::NewRoomOccupant {
                    room_id: room.id,
                    user_id: other.id,
                    can_send: true,
                    can_add_user: false,
                    can_change_name: false,
                };

                crate::actions::add_chat_room_occupants(&pg_conn, &[current_user, other_user])
                    .map_err(|e| ServerErrorTypes::from(e))
            }
        })
        .await
    );

    Ok(HttpResponse::Ok().into())
}

async fn room_info(
    req: HttpRequest,
    authorized: Authorized,
    root_logger: web::Data<Logger>,
    db_pool: web::Data<DbPool>,
    request: web::Json<request::GetRoomInfo>,
) -> Result<HttpResponse, HttpResponse> {
    let logger = root_logger.new(
        slog::o!("endpoint" =>  "/chat/room_info", "host" => req.connection_info().host().to_owned(), "user_id" => authorized.user.id.clone()),
    );

    // Since we don't expose that fact that room id's are actually integers providing a non-integer
    // room id is basically the same as providing a room that doesn't exist
    let room_id = request.room_id.parse().map_err(|_| {
        HttpResponse::NotFound()
            .json(json!({ "error": "Room not found", "error_code": "ROOM_NOT_FOUND" }))
    })?;

    if let Some(room) = try_or_log_error!(
        logger,
        web::block({
            let db_pool = db_pool.clone();
            move || {
                let conn = db_pool.get().map_err(|e| ServerErrorTypes::from(e))?;

                actions::try_get_chat_room(&conn, room_id).map_err(|e| ServerErrorTypes::from(e))
            }
        })
        .await
    ) {
        let occupants = try_or_log_error!(
            logger,
            web::block(move || {
                let conn = db_pool.get().map_err(|e| ServerErrorTypes::from(e))?;

                actions::get_room_occupants(&conn, room_id).map_err(|e| ServerErrorTypes::from(e))
            })
            .await
        );

        // If the user isn't part of a room we should just say that it's not found as they don't
        // have permission to know about it
        if occupants
            .iter()
            .find(|o| o.user_id == authorized.user.id)
            .is_none()
        {
            return Err(HttpResponse::NotFound()
                .json(json!({ "error": "Room not found", "error_code": "ROOM_NOT_FOUND" })));
        }

        return Ok(HttpResponse::Ok().json(response::RoomInfo {
            room_name: room.name,
            occupants: occupants.into_iter().map(|o| o.user_id).collect(),
        }));
    } else {
        // The provided room id does not exist in the database
        return Err(HttpResponse::NotFound()
            .json(json!({ "error": "Room not found", "error_code": "ROOM_NOT_FOUND" })));
    }
}
