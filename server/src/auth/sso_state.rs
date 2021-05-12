//! This file contains code relating to storing the SSO state tokens for
//! global access.

use super::AUTH_REDIS_PREFIX;
use crate::{try_or_log_error, RedisConnection};

use actix_web::HttpResponse;

use slog::{trace, warn, Logger};

use bb8_redis::redis;

/// Creates a new state id (guaranteed to be unique) and inserts the default SSO state (WAITING)
/// into the redis db. It then returns the generated state id.
/// Returns a properly formatted HttpResponse error if there is an issue.
pub async fn new_state(
    logger: &Logger,
    conn: &mut RedisConnection,
    timeout_at: u64,
) -> Result<String, HttpResponse> {
    use rand::Rng;
    let waiting_json = serde_json::to_string(&SSOStatus::WAITING).unwrap();

    // Loop until the random number doesn't exist.
    // This is very very likely to only loop once. The probability of generating
    // a random number that already exists is exceedingly low.
    loop {
        let mut rng = rand::thread_rng();
        // Generate a random u64 from the secure random source that thread_rng provides.
        let token: u64 = rng.gen();
        // Convert to upper case hex
        let hex = format!("{:X}", token);

        let redis_key = format!("{}{}", AUTH_REDIS_PREFIX, hex);

        // SETNX will insert the key if it doesn't exist or will return false if it did which will
        // mean that we loop around again
        if try_or_log_error!(
            logger,
            redis::cmd("SETNX")
                .arg(&[&redis_key, &waiting_json])
                .query_async(conn)
                .await,
            || format!("failed to insert state with key {}", hex)
        ) {
            try_or_log_error!(
                logger,
                redis::cmd("EXPIREAT")
                    .arg(&[redis_key, format!("{}", timeout_at)])
                    .query_async(conn)
                    .await,
                || format!(
                    "failed to set timeout on key {} (should have been at {} seconds since epoch)",
                    hex, timeout_at
                )
            );

            trace!(logger, "Generated and inserted key for SSO (`{}`)", hex);
            return Ok(hex);
        } else {
            warn!(
                logger,
                "state key `{}` already existed trying again, this is very low probability", hex
            );
        }
    }
}

/// Gets the status of the specified token or None if it doesn't exist.
/// This method will return properly formatted errors if there is a failure to connect to the
/// database.
pub async fn get_status(
    logger: &Logger,
    conn: &mut RedisConnection,
    token: String,
) -> Result<Option<SSOStatus>, HttpResponse> {
    let status: Option<String> = try_or_log_error!(
        logger,
        redis::cmd("GET")
            .arg(format!("{}{}", AUTH_REDIS_PREFIX, token))
            .query_async(conn)
            .await,
        || format!("failed to get status of key {}", token)
    );

    // We need to access the status if it's some and parse the JSON string to SSOStatus
    let status = if let Some(status_text) = status {
        Some(try_or_log_error!(
            logger,
            serde_json::from_str(&status_text),
            || format!("key `{}` contained improperly formated SSOStatus", token)
        ))
    } else {
        None
    };

    Ok(status)
}

/// Updates the status to a new value.
/// Returns a properly formatted HttpResponse on error.
pub async fn update_status(
    logger: &Logger,
    conn: &mut RedisConnection,
    token: String,
    new_status: SSOStatus,
) -> Result<(), HttpResponse> {
    try_or_log_error!(
        logger,
        redis::cmd("SET")
            .arg(&[
                format!("{}{}", AUTH_REDIS_PREFIX, token),
                serde_json::to_string(&new_status).unwrap()
            ])
            .query_async(conn)
            .await,
        || format!("failed to get status of key {}", token)
    );

    Ok(())
}

/// The status of a particular SSO request.
/// This doesn't encode `TIMED_OUT` because that is represented by there
/// not being a key for a particular state token in `SSOState`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SSOStatus {
    /// The SSO request has begun but the server hasn't received anything
    /// back to the callback.
    /// It has also not yet timed out.
    WAITING,
    /// The callback has been called but the server is still verifying the user.
    PROCESSING,
    /// An error has occured and this stored the associated human readable error
    /// message.
    ERROR { human_readable_message: String },
    /// The callback has been called and the user has been verified and is
    /// guaranteed to exist in the database at this point.
    /// The `auth_token` can be sent to the client app.
    AUTHENTICATED { auth_token: String },
}
