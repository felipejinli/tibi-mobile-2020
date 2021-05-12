table! {
    use crate::custom_schema_types::export::*;

    announcements (id) {
        id -> Int4,
        title -> Varchar,
        subtitle -> Varchar,
        image -> Varchar,
        visible -> Bool,
        created_at -> Timestamptz,
    }
}

table! {
    use crate::custom_schema_types::export::*;

    chat_history (msg_id) {
        msg_id -> Int4,
        poster -> Varchar,
        room -> Int4,
        posted_at -> Timestamptz,
        message -> Text,
    }
}

table! {
    use crate::custom_schema_types::export::*;

    chat_occupants (room_id, user_id) {
        room_id -> Int4,
        user_id -> Varchar,
        can_send -> Bool,
        can_add_user -> Bool,
        can_change_name -> Bool,
        last_received -> Nullable<Int4>,
        last_read -> Nullable<Int4>,
    }
}

table! {
    use crate::custom_schema_types::export::*;

    chat_room (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    use crate::custom_schema_types::export::*;

    events (id) {
        id -> Int4,
        pre_title -> Varchar,
        title -> Varchar,
        description -> Varchar,
        location -> Varchar,
        is_virtual -> Bool,
        virtual_link -> Nullable<Varchar>,
        price_pence -> Int4,
        images -> Array<Text>,
        lineup -> Array<Lineupitem>,
        visible -> Bool,
        event_start -> Timestamptz,
        created_at -> Timestamptz,
    }
}

table! {
    use crate::custom_schema_types::export::*;

    image_permissions (image_id, user_id) {
        image_id -> Varchar,
        user_id -> Varchar,
        can_write -> Bool,
        can_read -> Bool,
        can_remove -> Bool,
    }
}

table! {
    use crate::custom_schema_types::export::*;

    images (id) {
        id -> Varchar,
        private -> Bool,
        created_by -> Varchar,
        created_at -> Timestamptz,
        original_size_bytes -> Int4,
        optimised_size_bytes -> Int4,
    }
}

table! {
    use crate::custom_schema_types::export::*;

    users (id) {
        id -> Varchar,
        given_name -> Varchar,
        full_name -> Varchar,
        username -> Varchar,
        email -> Varchar,
        department -> Varchar,
        is_student -> Bool,
        is_admin -> Bool,
    }
}

joinable!(announcements -> images (image));
joinable!(chat_history -> chat_room (room));
joinable!(chat_history -> users (poster));
joinable!(chat_occupants -> chat_room (room_id));
joinable!(chat_occupants -> users (user_id));
joinable!(image_permissions -> images (image_id));
joinable!(image_permissions -> users (user_id));
joinable!(images -> users (created_by));

allow_tables_to_appear_in_same_query!(
    announcements,
    chat_history,
    chat_occupants,
    chat_room,
    events,
    image_permissions,
    images,
    users,
);
