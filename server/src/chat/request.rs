//! Contains the structs that represent requests to `/chat/*`

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateDM {
    pub other: String,
}

#[derive(Deserialize)]
pub struct GetRoomInfo {
    pub room_id: String,
}

#[derive(Deserialize)]
pub struct Retrieve {
    pub after: i32,
    // pub room_id: Option<i32>,
}
