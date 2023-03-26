// @generated automatically by Diesel CLI.

diesel::table! {
    messages (message_id) {
        message_id -> Int4,
        text -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        username -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(messages, users,);
