use crate::schema::*;
use diesel::{pg::data_types::PgTimestamp, AsChangeset, Insertable, Queryable};

pub use crate::custom_schema_types::LineupItem;

#[derive(Queryable)]
pub struct User {
    pub id: String,
    pub given_name: String,
    pub full_name: String,
    pub username: String,
    pub email: String,
    pub department: String,
    pub is_student: bool,
    pub is_admin: bool,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    /// This is the user's upi.
    pub id: String,
    pub given_name: String,
    pub full_name: String,
    pub username: String,
    pub email: String,
    pub department: String,
    pub is_student: bool,
}

#[derive(Queryable)]
pub struct Image {
    pub id: String,
    pub private: bool,
    pub created_by: String,
    pub created_at: PgTimestamp,
    // TODO: use this for faster image access / caching
    //    pub last_modified: String,
    pub original_size_bytes: i32,
    pub optimised_size_bytes: i32,
}

#[derive(Insertable)]
#[table_name = "images"]
pub struct NewImage {
    pub id: String,
    pub private: bool,
    pub created_by: String,
    pub original_size_bytes: i32,
    pub optimised_size_bytes: i32,
}

#[derive(Queryable)]
pub struct Announcement {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub image: String,
    pub visible: bool,
    pub created_at: PgTimestamp,
}

#[derive(Insertable)]
#[table_name = "announcements"]
pub struct NewAnnouncement {
    pub title: String,
    pub subtitle: String,
    pub image: String,
}

#[derive(Queryable)]
pub struct Event {
    pub id: i32,
    pub pre_title: String,
    pub title: String,
    pub description: String,
    pub location: String,
    pub is_virtual: bool,
    pub virtual_link: Option<String>,
    pub price_pence: i32,
    pub images: Vec<String>,
    pub lineup: Vec<LineupItem>,
    pub visible: bool,
    pub event_start: PgTimestamp,
    pub created_at: PgTimestamp,
}

#[derive(Insertable)]
#[table_name = "events"]
pub struct NewEvent {
    pub pre_title: String,
    pub title: String,
    pub description: String,
    pub location: String,
    pub is_virtual: bool,
    pub virtual_link: String,
    pub price_pence: i32,
    pub images: Vec<String>,
    pub lineup: Vec<LineupItem>,
    pub visible: bool,
    pub event_start: PgTimestamp,
}

#[derive(Queryable)]
pub struct ChatRoom {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Insertable)]
#[table_name = "chat_room"]
pub struct NewChatRoom {
    pub name: Option<String>,
}

#[derive(Queryable, Clone, Debug)]
pub struct RoomOccupant {
    pub room_id: i32,
    pub user_id: String,
    pub can_send: bool,
    pub can_add_user: bool,
    pub can_change_name: bool,
    pub last_received: Option<i32>,
    pub last_read: Option<i32>,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "chat_occupants"]
pub struct NewRoomOccupant {
    pub room_id: i32,
    pub user_id: String,
    pub can_send: bool,
    pub can_add_user: bool,
    pub can_change_name: bool,
}

#[derive(Queryable)]
pub struct ChatMessage {
    pub msg_id: i32,
    pub poster: String,
    pub room: i32,
    pub timestamp: PgTimestamp,
    pub message: String,
}

#[derive(Insertable)]
#[table_name = "chat_history"]
pub struct NewChatMessage {
    pub poster: String,
    pub room: i32,
    pub message: String,
}
