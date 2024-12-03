// @generated automatically by Diesel CLI.

diesel::table! {
    api_services (id) {
        id -> Int4,
        auth_service_id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        base_url -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    auth_service (id) {
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
    user_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        auth_service_id -> Int4,
        access_token -> Text,
        refresh_token -> Nullable<Text>,
        expires_at -> Timestamp,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        password_hash -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(api_services -> auth_service (auth_service_id));
diesel::joinable!(user_tokens -> auth_service (auth_service_id));
diesel::joinable!(user_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_services,
    auth_service,
    user_tokens,
    users,
);
