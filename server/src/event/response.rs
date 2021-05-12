//! This file contains the various structs that represent the responses sent
//! on the `/event/*` endpoints.

use serde::Serialize;

use super::LineupItem;
use crate::{model, util};

#[derive(Serialize)]
pub struct EventResponse {
    id: String,
    pre_title: String,
    title: String,
    description: String,
    location: String,
    is_virtual: bool,
    virtual_link: Option<String>,
    price_pence: i32,
    images: Vec<String>,
    lineup: Vec<LineupItem>,
    visible: bool,
    event_start: u64,
    created_at: u64,
}

impl From<model::Event> for EventResponse {
    fn from(event: model::Event) -> Self {
        let created_at = util::pgtimestamp_to_epoch_seconds(event.created_at);
        let event_start = util::pgtimestamp_to_epoch_seconds(event.event_start);

        EventResponse {
            id: format!("{}", event.id),
            pre_title: event.pre_title,
            title: event.title,
            description: event.description,
            location: event.location,
            is_virtual: event.is_virtual,
            virtual_link: event.virtual_link,
            price_pence: event.price_pence,
            images: event.images,
            lineup: event.lineup.into_iter().map(|item| item.into()).collect(),
            visible: event.visible,
            created_at,
            event_start,
        }
    }
}

#[derive(Serialize)]
pub struct EventList {
    events: Vec<EventResponse>,
}

impl From<Vec<model::Event>> for EventList {
    fn from(list: Vec<model::Event>) -> Self {
        EventList {
            events: list.into_iter().map(|a| EventResponse::from(a)).collect(),
        }
    }
}
