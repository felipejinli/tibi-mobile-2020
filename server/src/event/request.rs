//! This file contains the various structs that represent the requests sent
//! by the client to the `/event/*` endpoints.

use super::LineupItem;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewEvent {
    pub pre_title: String,
    pub title: String,
    pub description: String,
    pub lineup: Vec<LineupItem>,
    pub location: String,
    pub is_virtual: bool,
    pub virtual_link: String,
    pub price_pence: i32,
    pub images: Vec<String>,
    pub visible: bool,
    /// Unix timestamp
    pub event_start: u64,
}
