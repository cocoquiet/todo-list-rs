// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        completed -> Bool,
        created_at -> Timestamp,
    }
}
