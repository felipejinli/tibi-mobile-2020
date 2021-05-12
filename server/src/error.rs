//! This file contains common definitons and util methods for handling various errors
use actix_web::error::BlockingError;
use thiserror::Error;

/// This represents errors caused by various endpoints.
/// It's important that the full context of the error is logged internally but it's equally
/// important that the exact details are never leaked to the responses.
/// All the errors defined here map to various generic error responses (mostly
/// InternalServerError), but they also log the exact details for developers to debug.
#[derive(Error, Debug)]
pub enum ServerErrorTypes {
    /// Occurs when the server can't connect to the redis database
    #[error("redis connection failed: {}", _0)]
    RedisFailedConnection(#[from] bb8_redis::bb8::RunError<bb8_redis::redis::RedisError>),
    /// Occurs when the server can't connect to the postgres database
    #[error("postgres connection failed: {}", _0)]
    PostgresFailedConnection(#[from] diesel::r2d2::PoolError),
    /// Occurs when there's an error communicating with the redis database (not when opening a
    /// connection but when using one)
    #[error("redis communication failed: {}", _0)]
    RedisFailedCommunication(#[from] bb8_redis::redis::RedisError),
    /// Occurs when there's an error communicating with the postgres database (not when opening a
    /// connection but when using one)
    #[error("postgres communication failed: {}", _0)]
    PostgresFailedCommunication(#[from] diesel::result::Error),
    /// When there is serde JSON formatting error in internal data structures e.g. when stored in
    /// redis. Note: this data we're reading must have been originally formatted by the server.
    /// This error should not be used for general json formatting issues particularily not those
    /// from the client as this designates an internal server error.
    /// The formatting issues may be that the JSON is improper or that it did not represent the
    /// requried struct.
    #[error("internal JSON data structure could not be deserialized: {}", _0)]
    InvalidJSONStructure(#[from] serde_json::Error),
    /// When a blocking error occurs (BlockingError::Cancelled)
    #[error("thread pool is gone (blocking error: canceled)")]
    BlockingErrorCanceled,
}

impl<T: Into<ServerErrorTypes> + std::fmt::Debug> From<BlockingError<T>> for ServerErrorTypes {
    fn from(err: BlockingError<T>) -> Self {
        match err {
            BlockingError::Error(e) => e.into(),
            BlockingError::Canceled => ServerErrorTypes::BlockingErrorCanceled,
        }
    }
}

/// A helper function to log an error with some optional context and then return an
/// InternalServerError response properly formatted.
/// By making this into a macro it allows slog::error to keep the original source line of the
/// caller.
#[macro_export]
macro_rules! try_or_log_error {
    ($logger:expr, $result:expr, $context_fn:expr) => {
        $result.map_err(|error| {
            slog::error!($logger, "Internal error: {}: {}", $context_fn(), Into::<$crate::error::ServerErrorTypes>::into(error));

            actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "There was an internal server error, please try again later",
                "error_code": "INTERNAL_SERVER_ERROR"
            }))
        })?
    };
    ($logger:expr, $result:expr) => {
        $result.map_err(|error| {
            slog::error!($logger, "Internal error: {}", Into::<$crate::error::ServerErrorTypes>::into(error));

            actix_web::HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "There was an internal server error, please try again later",
                "error_code": "INTERNAL_SERVER_ERROR"
            }))
        })?
    };
}

/// Similar to [try_or_log_error] except that it instead maps to error instead of returning it.
/// You must provide the map function, it will not return an internal server error HttpResponse
/// unlike `try_or_log_error`.
#[macro_export]
macro_rules! map_log_error {
    ($logger:expr, $result:expr, $context_fn:expr; $map_err:expr) => {
        $result.map_err(|error| {
            let error = Into::<$crate::error::ServerErrorTypes>::into(error);
            slog::error!($logger, "Internal error: {}: {}", $context_fn(), error,);

            $map_err(error)
        })
    };
    ($logger:expr, $result:expr; $map_err:expr) => {
        $result.map_err(|error| {
            let error = Into::<$crate::error::ServerErrorTypes>::into(error);
            slog::error!($logger, "Internal error: {}", error);

            $map_err(error)
        })
    };
}
