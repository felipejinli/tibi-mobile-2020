//! The file contains the code for implemented the live part of chat.
//! Specifically the websocket actor that handles incomming and outgoing messages for the user
//! and coordinates with redis.

use crate::{map_log_error, DbPool, RedisPool};
use actix::{
    Actor, ActorContext, ActorFuture, AsyncContext, Handler, ResponseFuture, StreamHandler,
    WrapFuture,
};
use actix_web::{web, HttpResponse};
use actix_web_actors::ws;
use bb8_redis::redis::aio::PubSub;
use serde_json::json;

use std::sync::Arc;

use super::response::ChatMessage;

use slog::{error, trace, warn, Logger};

use super::REDIS_CHAT_PREFIX;

enum ChatState {
    /// Represents the state before correct authentication is provided
    Unauthenticated {
        jwt_secret: hmac::Hmac<sha2::Sha256>,
    },
    /// Represents the state when authentication has been successful
    Authenticated { user_id: String },
}

pub struct LiveChatHandler {
    // redis_channel: String,
    logger: Logger,
    postgres_pool: Arc<DbPool>,
    redis_pool: Arc<RedisPool>,
    state: ChatState,
}

impl LiveChatHandler {
    pub async fn new(
        logger: Logger,
        postgres_pool: Arc<DbPool>,
        redis_pool: Arc<RedisPool>,
        jwt_secret: hmac::Hmac<sha2::Sha256>,
    ) -> Result<Self, HttpResponse> {
        // let redis_channel = format!("{}{}", REDIS_CHAT_PREFIX, user_id);

        Ok(LiveChatHandler {
            redis_pool,
            logger,
            postgres_pool,
            state: ChatState::Unauthenticated { jwt_secret },
        })
    }
}

#[derive(Debug, Clone, serde::Deserialize, actix::Message)]
#[rtype("()")]
struct IncomingChatMessage {
    message: String,
    room: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct AuthenticationMessage {
    auth_token: String,
}

async fn setup_redis_conn(
    logger: Logger,
    redis_pool: Arc<RedisPool>,
    redis_channel: String,
) -> Result<PubSub, ()> {
    let conn = map_log_error!(logger, redis_pool.dedicated_connection().await, || {
        "Couldn't create dedicated connection for pubsub"
    }; |_| ())?;

    let mut pubsub = conn.into_pubsub();
    map_log_error!(logger, pubsub.subscribe(redis_channel).await, || {
        "couldn't subscribe to channel to listen for chat messages"
    }; |_| ())?;

    Ok(pubsub)
}

impl LiveChatHandler {
    fn on_authenticated(&mut self, user_id: String, ctx: &mut <Self as Actor>::Context) {
        ctx.spawn(
            async {}
                .into_actor(self)
                .then(move |_, this, _ctx| {
                    setup_redis_conn(
                        this.logger.clone(),
                        this.redis_pool.clone(),
                        format!("{}{}", REDIS_CHAT_PREFIX, user_id),
                    )
                    .into_actor(this)
                })
                .map(|res, act, ctx| {
                    if let Ok(pubsub) = res {
                        ctx.add_stream(pubsub.into_on_message());
                        trace!(act.logger, "listening for incoming messages");
                        ctx.text(
                            serde_json::to_string(
                                &json!({ "type": "status", "status": "CONNECTED" }),
                            )
                            .unwrap(),
                        );
                    } else {
                        trace!(
                            act.logger,
                            "due to earlier failure the channel is shutting down"
                        );
                        ctx.stop();
                    }
                }),
        );
    }
}

impl Actor for LiveChatHandler {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for LiveChatHandler {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => match &self.state {
                ChatState::Authenticated { .. } => {
                    if let Ok(msg) = serde_json::from_str::<IncomingChatMessage>(&text) {
                        ctx.address().do_send(msg);
                    } else {
                        warn!(self.logger, "client sent invalid message: {}", text);
                        return;
                    }
                }
                ChatState::Unauthenticated { jwt_secret } => {
                    if let Ok(msg) = serde_json::from_str::<AuthenticationMessage>(&text) {
                        match crate::auth::jwt::verify_token(&jwt_secret, &msg.auth_token) {
                            Ok(user_id) => {
                                self.logger = self
                                    .logger
                                    .new(slog::o!("state" => "authenticated", "user_id" => user_id.clone()));
                                trace!(self.logger, "user authenticated for live chat");
                                self.on_authenticated(user_id.clone(), ctx);
                                self.state = ChatState::Authenticated { user_id };
                            }
                            Err(err) => {
                                trace!(
                                    self.logger,
                                    "invalid auth token, closing channel with error: {:?}",
                                    err
                                );
                                ctx.text(
                                    serde_json::to_string(&json!({
                                        "error": err.as_error(),
                                        "error_code": err.as_error_code()
                                    }))
                                    .unwrap(),
                                );
                                ctx.stop();
                            }
                        }
                    } else {
                        warn!(
                            self.logger,
                            "client sent invalid authentication message: {}", text
                        );
                        return;
                    }
                }
            },
            Ok(ws::Message::Binary(_)) => warn!(self.logger, "Tried to send binary data"),
            _ => (),
        }
    }
}

impl StreamHandler<bb8_redis::redis::Msg> for LiveChatHandler {
    fn handle(&mut self, msg: bb8_redis::redis::Msg, ctx: &mut Self::Context) {
        let text: String = match msg.get_payload() {
            Ok(text) => text,
            Err(e) => {
                error!(
                    self.logger,
                    "couldn't get payload from pubsub message: {}", e
                );
                return;
            }
        };

        let message = match serde_json::from_str::<ChatMessage>(&text) {
            Ok(message) => message,
            Err(_) => {
                error!(
                    self.logger,
                    "incoming redis sub message was not a ChatMessage encoded in JSON got: {}",
                    text
                );
                return;
            }
        };

        ctx.text(serde_json::to_string(&json!({ "type": "message", "message": message })).unwrap());
    }
}

impl Handler<IncomingChatMessage> for LiveChatHandler {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, msg: IncomingChatMessage, _ctx: &mut Self::Context) -> Self::Result {
        let logger = self.logger.clone();
        let user_id = if let ChatState::Authenticated { user_id } = &self.state {
            Some(user_id.clone())
        } else {
            error!(logger, "state wasn't authenticated even though it should be impossible to get to this state if the user is unauthenticated");
            None
        };
        let redis_pool = Arc::clone(&self.redis_pool);
        let postgres_pool = Arc::clone(&self.postgres_pool);

        Box::pin(async {
            // This will not be true if there was some issue and the state was unauthenticated, in
            // that case we just do nothing (the error is logged above)
            if let Some(user_id) = user_id {
                // Ignore error as it's logged inside the method
                let _ = handle_message(logger, user_id, redis_pool, postgres_pool, msg).await;
            }
        })
    }
}

async fn handle_message(
    logger: Logger,
    user_id: String,
    redis_pool: Arc<RedisPool>,
    pg_pool: Arc<DbPool>,
    incoming: IncomingChatMessage,
) -> Result<(), ()> {
    let pg_conn = web::block({
        let pool = pg_pool.clone();
        move || pool.get()
    })
    .await
    .map_err(|e| {
        error!(
            logger,
            "couldn't get a postgres connection for sending a chat message: {}", e
        );
    })?;

    let room: i32 = incoming.room.parse().map_err(|e| {
        warn!(
            logger,
            "client sent invalid group id (couldn't parse into i32): {}", e
        );
    })?;

    let occupants = web::block(move || crate::actions::get_room_occupants(&pg_conn, room))
        .await
        .map_err(|e| {
            error!(
                logger,
                "couldn't get room occupants in room {}: {}", room, e
            );
        })?;

    if let Some(sender) = occupants.iter().find(|o| o.user_id == user_id) {
        if !sender.can_send {
            error!(
                logger,
                "user tried to send message to room {} and they lacked send permissions ({:?})",
                room,
                sender,
            );

            return Err(());
        }
    } else {
        error!(
            logger,
            "user tried to send message to room {} which is was not an occupant of", room,
        );

        return Err(());
    }

    let message = crate::model::NewChatMessage {
        poster: user_id,
        room,
        message: incoming.message,
    };

    let message = map_log_error!(logger, web::block({
        let pool = pg_pool.clone();
        move || -> Result<_, crate::error::ServerErrorTypes> {
            let conn = pool.get()?;
            Ok(crate::actions::post_message(&conn, message)?)
        }
    }).await; |_| ())?;

    let msg_id = message.msg_id;

    // Convert from database record representation to a struct that can be serialized and sent to
    // the user
    let message: ChatMessage = message.into();

    let message_json = serde_json::to_string(&message).unwrap();

    let mut redis_conn = map_log_error!(logger, redis_pool.get().await; |_| ())?;
    let c: &mut crate::RedisConnection = &mut redis_conn;

    let mut received: i32 = 0;
    for occupant in occupants {
        received += map_log_error!(logger, bb8_redis::redis::Cmd::publish(
            format!("{}{}", REDIS_CHAT_PREFIX, occupant.user_id),
            &message_json,
        )
        .query_async::<_, i32>(c)
        .await; |_| ())?;
    }

    slog::trace!(
        logger,
        "{} subscribers received chat message id `{}` from current user",
        received,
        msg_id,
    );
    Ok(())
}
