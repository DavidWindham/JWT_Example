// @generated automatically by Diesel CLI.

diesel::table! {
    refresh_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Varchar,
        valid_until -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(refresh_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(refresh_tokens, users,);
