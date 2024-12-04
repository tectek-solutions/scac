// @generated automatically by Diesel CLI.

diesel::table! {
    authentifications (id) {
        id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        auth_url -> Text,
        token_url -> Text,
        client_id -> Text,
        client_secret -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        password_hash -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    authentifications,
    users,
);
