// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        description -> Text,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        display_name -> Text,
        username -> Text,
        email -> Text,
        password -> Text,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
