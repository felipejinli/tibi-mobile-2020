//! This file contains the various structs that represent the responses sent
//! on the `/auth/*` endpoints.

use serde::Serialize;

#[derive(Serialize)]
pub struct SSOResponse {
    pub redirect_url: String,
    pub check_code: String,
    pub timeout: u64,
}

#[derive(Serialize)]
pub struct RefreshResponse {
    pub auth_token: String,
}
