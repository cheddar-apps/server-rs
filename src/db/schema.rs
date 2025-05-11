// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "channel_type"))]
    pub struct ChannelType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ChannelType;

    channels (id) {
        id -> Int8,
        server_id -> Int8,
        #[max_length = 72]
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> ChannelType,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    messages (id) {
        id -> Int8,
        author_id -> Int8,
        channel_id -> Int8,
        content -> Jsonb,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    servers (id) {
        id -> Int8,
        #[max_length = 72]
        name -> Varchar,
        owner_uid -> Int8,
        users -> Array<Nullable<Int8>>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    user_authentication (user_id) {
        user_id -> Int8,
        #[max_length = 60]
        password_hash -> Varchar,
        mfa_enabled -> Bool,
        mfa_b32 -> Nullable<Bytea>,
        mfa_auth_url -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    user_tokens (token) {
        token -> Text,
        user_id -> Int8,
        scopes -> Array<Nullable<Text>>,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Int8,
        #[max_length = 40]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        servers -> Array<Nullable<Int8>>,
    }
}

diesel::joinable!(channels -> servers (server_id));
diesel::joinable!(messages -> channels (channel_id));
diesel::joinable!(messages -> users (author_id));
diesel::joinable!(servers -> users (owner_uid));
diesel::joinable!(user_authentication -> users (user_id));
diesel::joinable!(user_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    channels,
    messages,
    servers,
    user_authentication,
    user_tokens,
    users,
);
