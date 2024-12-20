// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
    }
}
