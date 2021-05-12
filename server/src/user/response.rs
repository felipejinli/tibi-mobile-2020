//! This file contains the various structs that represent the responses sent
//! on the `/user/*` endpoints.

use serde::Serialize;

use crate::model;

/// Represents the data returned when returning a user.
/// It is important that this is separate from the user in `model` because
/// if we sent that directly we may leak sensitive data if we add extra
/// secret fields to the user in the future.
/// It is best practise to manually specify exactly what we want to send here.
#[derive(Serialize)]
pub struct UserResponse {
    id: String,
    given_name: String,
    full_name: String,
    username: String,
    email: String,
    department: String,
}

impl From<model::User> for UserResponse {
    fn from(user: model::User) -> UserResponse {
        UserResponse {
            id: user.id,
            given_name: user.given_name,
            full_name: user.full_name,
            username: user.username,
            email: user.email,
            department: user.department,
        }
    }
}
