// @generated automatically by Diesel CLI.

diesel::table! {
    actions (id) {
        id -> Int4,
        api_id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 8]
        http_method -> Varchar,
        http_endpoint -> Text,
        http_parameters -> Nullable<Jsonb>,
        http_headers -> Nullable<Jsonb>,
        http_body -> Nullable<Jsonb>,
        trigger_date_json_path -> Text,
        trigger_date_format -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    apis (id) {
        id -> Int4,
        authentication_id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        base_url -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    authentications (id) {
        id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        authentication_url -> Text,
        refresh_token_url -> Text,
        access_token_json_path -> Text,
        refresh_token_json_path -> Text,
        access_token_expires_at_json_path -> Text,
        refresh_token_expires_at_json_path -> Text,
        is_expires_at_relative -> Bool,
        client_id -> Text,
        client_secret -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    reactions (id) {
        id -> Int4,
        api_id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 8]
        http_method -> Varchar,
        http_endpoint -> Text,
        http_parameters -> Nullable<Jsonb>,
        http_headers -> Nullable<Jsonb>,
        http_body -> Nullable<Jsonb>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    triggers (id) {
        id -> Int4,
        workflow_id -> Int4,
        data -> Nullable<Jsonb>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_tokens (id) {
        id -> Int4,
        users_id -> Int4,
        authentication_id -> Int4,
        access_token -> Text,
        access_token_expires_at -> Timestamp,
        refresh_token -> Text,
        refresh_token_expires_at -> Timestamp,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    workflows (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        description -> Nullable<Text>,
        action_id -> Int4,
        reaction_id -> Int4,
        data_transformation -> Nullable<Jsonb>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(actions -> apis (api_id));
diesel::joinable!(apis -> authentications (authentication_id));
diesel::joinable!(reactions -> apis (api_id));
diesel::joinable!(triggers -> workflows (workflow_id));
diesel::joinable!(user_tokens -> authentications (authentication_id));
diesel::joinable!(user_tokens -> users (users_id));
diesel::joinable!(workflows -> actions (action_id));
diesel::joinable!(workflows -> reactions (reaction_id));
diesel::joinable!(workflows -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    actions,
    apis,
    authentications,
    reactions,
    triggers,
    user_tokens,
    users,
    workflows,
);
