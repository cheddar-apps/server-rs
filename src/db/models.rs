use super::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

#[derive(Debug, DbEnum, Clone)]
#[ExistingTypePath = "crate::db::schema::sql_types::ChannelType"]
pub enum ChannelType {
    Text,
    Voice,
}

#[derive(
    Queryable, Selectable, Insertable, Associations, Identifiable, Debug, Clone, AsChangeset,
)]
#[diesel(table_name = channels)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Server, foreign_key = server_id))]
pub struct Channel {
    pub id: i64,
    pub server_id: i64,
    pub name: String,
    pub type_: ChannelType,
}
#[derive(Queryable, Selectable, Insertable, Identifiable, Debug, Clone, AsChangeset)]
#[diesel(table_name = servers)]
#[diesel(primary_key(id))]
pub struct Server {
    pub id: i64,
    pub name: String,
    pub owner_uid: i64,
    pub users: Vec<Option<i64>>, // TODO: figure out why these are nullable
}
#[derive(
    Queryable, Selectable, Insertable, Associations, Identifiable, Debug, Clone, AsChangeset,
)]
#[diesel(table_name = messages)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(belongs_to(Channel, foreign_key = channel_id))]
pub struct Message {
    pub id: i64,
    pub author_id: i64,
    pub channel_id: i64,
    pub content: serde_json::Value,
}

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug, Clone, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub servers: Vec<Option<i64>>,
}
#[derive(
    Queryable, Selectable, Insertable, Associations, Identifiable, Debug, Clone, AsChangeset,
)]
#[diesel(table_name = user_tokens)]
#[diesel(primary_key(token))]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct UserToken {
    pub token: String,
    pub user_id: i64,
    pub scopes: Vec<Option<String>>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

#[derive(
    Queryable, Selectable, Insertable, Associations, Identifiable, Debug, Clone, AsChangeset,
)]
#[diesel(table_name = user_authentication)]
#[diesel(primary_key(user_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct UserAuthentication {
    pub user_id: i64,
    pub password_hash: String,
    pub mfa_enabled: bool,
    pub mfa_b32: Option<Vec<u8>>,
    pub mfa_auth_url: Option<String>,
}
