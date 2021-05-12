//! This files contains the reponses from `/chat/*` endpoints (including websocket)

use serde::Serialize;

use crate::model;

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
/// A serializable chat message that can be sent to the client
pub struct ChatMessage {
    id: i32,
    poster: String,
    room: String,
    message: String,
    timestamp: u64,
}

impl From<model::ChatMessage> for ChatMessage {
    fn from(msg: model::ChatMessage) -> ChatMessage {
        let timestamp = crate::util::pgtimestamp_to_epoch_seconds(msg.timestamp);

        ChatMessage {
            id: msg.msg_id,
            poster: msg.poster,
            room: format!("{}", msg.room),
            message: msg.message,
            timestamp,
        }
    }
}

#[derive(Serialize)]
pub struct RoomInfo {
    pub room_name: Option<String>,
    pub occupants: Vec<String>,
}

#[derive(Serialize)]
pub struct Retrieve {
    pub messages: Vec<ChatMessage>,
}

impl From<Vec<model::ChatMessage>> for Retrieve {
    fn from(messages: Vec<model::ChatMessage>) -> Self {
        Retrieve {
            messages: messages.into_iter().map(|m| m.into()).collect(),
        }
    }
}
