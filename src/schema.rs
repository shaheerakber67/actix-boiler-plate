// @generated automatically by Diesel CLI.

diesel::table! {
    field_sector (id) {
        id -> Int4,
        created_by -> Nullable<Int4>,
        last_modified_by -> Nullable<Int4>,
        created_date -> Nullable<Timestamp>,
        last_modified_date -> Nullable<Timestamp>,
        status -> Nullable<Varchar>,
    }
}

diesel::table! {
    field_sector_translation (id) {
        id -> Int4,
        field_sector_id -> Int4,
        created_by -> Nullable<Int4>,
        last_modified_by -> Nullable<Int4>,
        created_date -> Nullable<Timestamp>,
        last_modified_date -> Nullable<Timestamp>,
        lang_code -> Varchar,
        name -> Varchar,
    }
}

diesel::table! {
    meeting (id) {
        id -> Int4,
        room_id -> Int4,
        lang_code -> Varchar,
        name -> Varchar,
    }
}

diesel::table! {
    room (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    user_type (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(field_sector_translation -> field_sector (field_sector_id));
diesel::joinable!(meeting -> room (room_id));
diesel::joinable!(user_type -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    field_sector,
    field_sector_translation,
    meeting,
    room,
    user_type,
    users,
);
