use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub servers: Vec<Option<i32>>
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::schema::servers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
pub struct Server {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub users: Vec<Option<i32>> // TODO(i3vie): WTF? This is not nullable in the sql?
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = crate::schema::channels)]
#[diesel(belongs_to(Server))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Channel {
    pub id: i32,
    pub name: String,
    pub server_id: i32,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Channel))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub channel_id: i32,
    pub author_id: i32,
}