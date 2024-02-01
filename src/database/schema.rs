// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        display_name -> Text,
        username -> Text,
        email -> Text,
        password -> Text,
    }
}
