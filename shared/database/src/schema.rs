// @generated automatically by Diesel CLI.

// pub mod sql_types {
//     #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
//     #[diesel(postgres_type(name = "http_method_enum"))]
//     pub struct HttpMethodEnum;

//     #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
//     #[diesel(postgres_type(name = "status_enum"))]
//     pub struct StatusEnum;
// }

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum HttpMethodEnum {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum StatusEnum {
    Active,
    Inactive,
    Pending,
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::HttpMethodEnum;

    actions (id) {
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
    authentification (id) {
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
    use diesel::sql_types::*;
    use crate::schema::HttpMethodEnum;

    reactions (id) {
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
    use crate::schema::StatusEnum;

    triggers (id) {
        id -> Int4,
        workflow_id -> Int4,
        data -> Jsonb,
        status -> StatusEnum,
        created_at -> Nullable<Timestamp>,
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

diesel::joinable!(actions -> api_services (api_service_id));
diesel::joinable!(api_services -> authentification (auth_service_id));
diesel::joinable!(reactions -> api_services (api_service_id));
diesel::joinable!(triggers -> workflows (workflow_id));
diesel::joinable!(user_tokens -> authentification (auth_service_id));
diesel::joinable!(user_tokens -> users (user_id));
diesel::joinable!(workflows -> actions (action_id));
diesel::joinable!(workflows -> reactions (reaction_id));
diesel::joinable!(workflows -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    actions,
    api_services,
    authentification,
    reactions,
    triggers,
    user_tokens,
    users,
    workflows,
);
