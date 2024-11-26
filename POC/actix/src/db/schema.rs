// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Text,
        name -> Text,
        description -> Text,
    }
}
