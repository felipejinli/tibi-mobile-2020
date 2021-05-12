//! This file contains the various structs that represent the requests sent
//! by the client to the `/announcement/*` endpoints.

use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewAnnouncement {
    pub title: String,
    pub subtitle: String,
    pub image: String,
}
