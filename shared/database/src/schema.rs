// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "http_method_enum"))]
    pub struct HttpMethodEnum;
}

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
    use diesel::sql_types::*;
    use super::sql_types::HttpMethodEnum;

    api_services_actions (id) {
        id -> Int4,
        api_service_id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        description -> Nullable<Text>,
        endpoint -> Text,
        method -> HttpMethodEnum,
        headers -> Nullable<Jsonb>,
        params -> Nullable<Jsonb>,
        json_path -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::HttpMethodEnum;

    api_services_reactions (id) {
        id -> Int4,
        api_service_id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        description -> Nullable<Text>,
        endpoint -> Text,
        method -> HttpMethodEnum,
        headers -> Nullable<Jsonb>,
        params -> Nullable<Jsonb>,
        json_path -> Nullable<Text>,
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
diesel::joinable!(api_services_actions -> api_services (api_service_id));
diesel::joinable!(api_services_reactions -> api_services (api_service_id));
diesel::joinable!(user_tokens -> auth_service (auth_service_id));
diesel::joinable!(user_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_services,
    api_services_actions,
    api_services_reactions,
    auth_service,
    user_tokens,
    users,
);
