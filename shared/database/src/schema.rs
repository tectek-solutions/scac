// @generated automatically by Diesel CLI.

diesel::table! {
    actions (id) {
        id -> Int4,
        apis_id -> Int4,
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
    apis (id) {
        id -> Int4,
        authentications_id -> Int4,
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
        client_id -> Text,
        client_secret -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    reactions (id) {
        id -> Int4,
        apis_id -> Int4,
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
        workflows_id -> Int4,
        data -> Nullable<Jsonb>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_tokens (id) {
        id -> Int4,
        users_id -> Int4,
        authentications_id -> Int4,
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
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 320]
        email -> Varchar,
        #[max_length = 60]
        password_hash -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    workflows (id) {
        id -> Int4,
        users_id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        description -> Nullable<Text>,
        actions_id -> Int4,
        reactions_id -> Int4,
        data_transformation -> Nullable<Jsonb>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(actions -> apis (apis_id));
diesel::joinable!(apis -> authentications (authentications_id));
diesel::joinable!(reactions -> apis (apis_id));
diesel::joinable!(triggers -> workflows (workflows_id));
diesel::joinable!(user_tokens -> authentications (authentications_id));
diesel::joinable!(user_tokens -> users (users_id));
diesel::joinable!(workflows -> actions (actions_id));
diesel::joinable!(workflows -> reactions (reactions_id));
diesel::joinable!(workflows -> users (users_id));

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
