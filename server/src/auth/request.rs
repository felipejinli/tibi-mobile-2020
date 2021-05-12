//! This file contains the various structs that represent the requests sent
//! by the client to the `/auth/*` endpoints.

use serde::Deserialize;

#[derive(Deserialize)]
pub struct SSOCheck {
    pub check_code: String,
}

#[derive(Deserialize, Debug)]
pub struct SSOCallback {
    pub state: String,
    pub result: String,
    pub code: String,
    pub client_id: String,
}

/// Comes from the UCL api.
#[derive(Deserialize)]
pub struct TokenApi {
    pub scope: String,
    pub state: String,
    pub ok: bool,
    pub client_id: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct UserData {
    pub department: String,
    pub email: String,
    pub full_name: String,
    pub cn: String,
    pub given_name: String,
    pub upi: String,
    pub ok: bool,
    pub is_student: bool,
    pub scope_number: i32,
}
