//! In order for easy usage of authentication a middleware is provided that can be optionally
//! enabled in order to check for valid authentication.

use crate::{actions, error::ServerErrorTypes, model, try_or_log_error, Config, DbPool};

use actix_web::{
    dev::Payload, http::HeaderValue, web, web::Data, FromRequest, HttpRequest, HttpResponse,
};

use slog::{warn, Logger};

use std::pin::Pin;

/// Represents an authorized connection with a given `user`.
pub struct Authorized {
    pub user: model::User,
}

/// Represent an error that could occur when trying to authenticate an endpoint
#[derive(Debug)]
pub enum AuthorizationError {
    MissingAuthToken,
    InvalidToken,
    TokenExpired,
}

impl AuthorizationError {
    pub fn as_error_code(&self) -> &'static str {
        use AuthorizationError::*;
        match self {
            MissingAuthToken => "AUTH_TOKEN_MISSING",
            InvalidToken => "AUTH_TOKEN_INVALID",
            TokenExpired => "AUTH_TOKEN_EXPIRED",
        }
    }

    pub fn as_error(&self) -> &'static str {
        use AuthorizationError::*;
        match self {
            MissingAuthToken => "You are not logged in",
            InvalidToken => "Your authentication token is invalid",
            TokenExpired => "Your login session has expired",
        }
    }
}

impl Into<HttpResponse> for AuthorizationError {
    fn into(self) -> HttpResponse {
        HttpResponse::Unauthorized().json(
            serde_json::json!({ "error": self.as_error(), "error_code": self.as_error_code() }),
        )
    }
}

impl FromRequest for Authorized {
    type Error = HttpResponse;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let config: &Data<Config> = req
            .app_data()
            .expect("Authentication middleware requires the app config in order to check tokens");
        let pool: &Data<DbPool> = req
            .app_data()
            .expect("Authentication middleware requires the db pool");
        let root_logger: &Data<Logger> = req
            .app_data()
            .expect("Authentication middleware requires the root logger");

        let logger = root_logger
            .new(slog::o!("endpoint" => req.path().to_owned(), "host" => req.peer_addr()));

        let auth_header = req.headers().get("Authorization").cloned();

        Box::pin(from_request(
            logger.clone(),
            config.clone(),
            pool.clone(),
            auth_header,
        ))
    }
}

async fn from_request(
    logger: Logger,
    config: Data<Config>,
    pool: Data<DbPool>,
    auth_header: Option<HeaderValue>,
) -> Result<Authorized, HttpResponse> {
    let secret = &config.jwt_secret;

    if let Some(header) = auth_header {
        let header = match header.to_str() {
            Ok(header) => header,
            // Header was not valid ascii
            Err(_) => {
                warn!(logger, "header was not valid ascii");
                return Err(AuthorizationError::InvalidToken.into());
            }
        };

        if header.len() < 8 {
            // Malformatted
            warn!(logger, "header length too short");
            return Err(AuthorizationError::InvalidToken.into());
        }

        if &header[0..7] != "Bearer " {
            warn!(logger, "header doesn't start with `Bearer `");
            return Err(AuthorizationError::InvalidToken.into());
        }

        // Auth token is everything after the "Bearer "
        let auth_token = &header[7..];

        match crate::auth::jwt::verify_token(secret, auth_token) {
            Ok(user_id) => {
                Ok(try_or_log_error!(
                    logger,
                    web::block({
                        let logger = logger.clone();
                        move || {
                            let conn = pool.get().map_err(|e| ServerErrorTypes::from(e))?;
                            match actions::get_user_by_id(&conn, &user_id) {
                                Ok(user) => Ok(Authorized { user }),
                                Err(e) => {
                                    // This error may be caused by a database issue not just that there
                                    // wasn't a user, maybe consider using try_get_user_by_id instead
                                    warn!(
                                        logger,
                                        "Received valid auth token with non existant user id {}",
                                        user_id
                                    );
                                    Err(ServerErrorTypes::from(e))
                                }
                            }
                        }
                    })
                    .await
                ))
            }
            Err(e) => Err(e.into()),
        }
    } else {
        Err(AuthorizationError::MissingAuthToken.into())
    }
}
