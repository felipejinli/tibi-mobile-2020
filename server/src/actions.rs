//! This file contains helper functions for operating on the database.

use diesel::pg::expression::dsl::*;
use diesel::prelude::*;
use diesel::result::{Error as DieselError, OptionalExtension};

use crate::{model::*, schema::*};

/// Inserts a user into the database if they don't exist, otherwise it will update an existing user
/// with the new fields from the provided field while retaining any unspecified fields. (vague fix
/// this.)
/// A user is considered identical if their `id` (aka upi) matches.
pub fn upsert_user(conn: &PgConnection, user: NewUser) -> Result<(), DieselError> {
    diesel::insert_into(users::table)
        .values(&user)
        .on_conflict(users::id)
        .do_update()
        .set(&user)
        .execute(conn)
        .map(|_| ())
}

pub fn get_user_by_id(conn: &PgConnection, id: &String) -> Result<User, DieselError> {
    users::table.find(id).first(conn)
}

pub fn try_get_user_by_id(conn: &PgConnection, id: &String) -> Result<Option<User>, DieselError> {
    users::table.find(id).first(conn).optional()
}

// Image:

pub fn register_image(conn: &PgConnection, image: NewImage) -> Result<usize, DieselError> {
    diesel::insert_into(images::table)
        .values(&image)
        .execute(conn)
}

pub fn get_image_by_id(conn: &PgConnection, id: &String) -> Result<Image, DieselError> {
    images::table.find(id).first(conn)
}

pub fn get_images_by_ids(conn: &PgConnection, id: Vec<String>) -> Result<Vec<Image>, DieselError> {
    images::table
        .filter(images::columns::id.eq(any(id)))
        .get_results(conn)
}

// Announcement:

pub fn new_announcement(
    conn: &PgConnection,
    announcement: NewAnnouncement,
) -> Result<Announcement, DieselError> {
    diesel::insert_into(announcements::table)
        .values(&announcement)
        .get_result(conn)
}

pub fn get_announcement_by_id(conn: &PgConnection, id: &i32) -> Result<Announcement, DieselError> {
    announcements::table.find(id).first(conn)
}

pub fn list_visible_announcements(conn: &PgConnection) -> Result<Vec<Announcement>, DieselError> {
    announcements::table
        .filter(announcements::columns::visible.eq(true))
        .order(announcements::columns::created_at.asc())
        .get_results(conn)
}

// Event:

pub fn new_event(conn: &PgConnection, event: NewEvent) -> Result<Event, DieselError> {
    diesel::insert_into(events::table)
        .values(&event)
        .get_result(conn)
}

pub fn get_event_by_id(conn: &PgConnection, id: &i32) -> Result<Event, DieselError> {
    events::table.find(id).first(conn)
}

pub fn list_visible_events(conn: &PgConnection) -> Result<Vec<Event>, DieselError> {
    events::table
        .filter(events::columns::visible.eq(true))
        .order(events::columns::created_at.asc())
        .get_results(conn)
}

// Chat:

/// Create a new chat room. This method does not add the user to the occupants list. That must be
/// done separately.
pub fn new_chat_room(conn: &PgConnection, room: NewChatRoom) -> Result<ChatRoom, DieselError> {
    diesel::insert_into(chat_room::table)
        .values(&room)
        .get_result(conn)
}

pub fn join_chat_room(
    conn: &PgConnection,
    occupant: NewRoomOccupant,
) -> Result<RoomOccupant, DieselError> {
    diesel::insert_into(chat_occupants::table)
        .values(&occupant)
        .get_result(conn)
}

pub fn try_get_chat_room(
    conn: &PgConnection,
    room_id: i32,
) -> Result<Option<ChatRoom>, DieselError> {
    chat_room::table.find(room_id).first(conn).optional()
}

pub fn add_chat_room_occupants(
    conn: &PgConnection,
    occupants: &[NewRoomOccupant],
) -> Result<Vec<RoomOccupant>, DieselError> {
    diesel::insert_into(chat_occupants::table)
        .values(occupants)
        .get_results(conn)
}

pub fn get_room_occupants(
    conn: &PgConnection,
    room_id: i32,
) -> Result<Vec<RoomOccupant>, DieselError> {
    chat_occupants::table
        .filter(chat_occupants::columns::room_id.eq(room_id))
        .get_results(conn)
}

pub fn post_message(
    conn: &PgConnection,
    message: NewChatMessage,
) -> Result<ChatMessage, DieselError> {
    diesel::insert_into(chat_history::table)
        .values(&message)
        .get_result(conn)
}

pub fn get_user_messages_after(
    conn: &PgConnection,
    user_id: &String,
    after: i32,
) -> Result<Vec<ChatMessage>, DieselError> {
    chat_occupants::table
        .inner_join(chat_history::table.on(chat_occupants::room_id.eq(chat_history::room)))
        .filter(chat_occupants::user_id.eq(user_id))
        .filter(chat_history::msg_id.gt(after))
        .select(chat_history::all_columns)
        .load(conn)
}

// pub fn get_messages_after(
//     conn: &PgConnection,
//     user_id: &String,
//     timestamp: PgTimestamp,
// ) -> Result<Vec<ChatMessage>, DieselError> {
//     chat_history::table.filter(chat_history::columns::che)
//
// }
