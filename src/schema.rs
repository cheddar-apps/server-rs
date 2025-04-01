// @generated automatically by Diesel CLI.

diesel::table! {
    channels (id) {
        id -> Int4,
        server_id -> Int4,
        #[max_length = 72]
        name -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        author_id -> Int4,
        channel_id -> Int4,
        content -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    servers (id) {
        id -> Int4,
        #[max_length = 72]
        name -> Varchar,
        user_id -> Int4,
        users -> Array<Nullable<Int4>>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 60]
        password_hash -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        servers -> Array<Nullable<Int4>>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(channels -> servers (server_id));
diesel::joinable!(messages -> channels (channel_id));
diesel::joinable!(messages -> users (author_id));
diesel::joinable!(servers -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    channels,
    messages,
    servers,
    users,
);
