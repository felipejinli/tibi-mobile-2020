//! This file contains the various structs that represent the responses sent
//! on the `/announcement/*` endpoints.

use serde::Serialize;

use crate::{model, util};

#[derive(Serialize)]
pub struct AnnouncementResponse {
    id: String,
    title: String,
    subtitle: String,
    image: String,
    created_at: u64,
}

impl From<model::Announcement> for AnnouncementResponse {
    fn from(announcement: model::Announcement) -> Self {
        AnnouncementResponse {
            id: format!("{}", announcement.id),
            title: announcement.title,
            subtitle: announcement.subtitle,
            image: announcement.image,
            created_at: util::pgtimestamp_to_epoch_seconds(announcement.created_at),
        }
    }
}

#[derive(Serialize)]
pub struct AnnouncementList {
    announcements: Vec<AnnouncementResponse>,
}

impl From<Vec<model::Announcement>> for AnnouncementList {
    fn from(list: Vec<model::Announcement>) -> Self {
        AnnouncementList {
            announcements: list
                .into_iter()
                .map(|a| AnnouncementResponse::from(a))
                .collect(),
        }
    }
}
