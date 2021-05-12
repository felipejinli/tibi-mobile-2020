//! This provides common code for operating on jwt (json web tokens) which is what the `AUTH_TOKEN`
//! used to verify users is based on.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use hmac::Hmac;
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

use crate::auth::middleware::AuthorizationError;

/// Represents the claims and are stored signed in the jwt.
#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    expires: u64,
    id: String,
}

/// Creates a new signed token valid for the user_id for the duration specified in lifetime
/// (from now to now + duration).
pub fn create_token(secret: &Hmac<Sha256>, user_id: String, lifetime: Duration) -> String {
    let expires = (SystemTime::now() + lifetime)
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let claims = Claims {
        expires: expires.as_secs(),
        id: user_id,
    };

    claims.sign_with_key(secret).expect("Key sign failed")
}

/// Checks to see if the token is both validly signed and hasn't expired.
/// It will return the user_id stored in the token if it is `Ok(token)`.
/// The error is a human readable error message.
pub fn verify_token(secret: &Hmac<Sha256>, token: &str) -> Result<String, AuthorizationError> {
    let claims: Claims = token
        .verify_with_key(secret)
        .map_err(|_| AuthorizationError::InvalidToken)?;

    if SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        > Duration::from_secs(claims.expires)
    {
        Err(AuthorizationError::TokenExpired)
    } else {
        Ok(claims.id)
    }
}
