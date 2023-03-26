use crate::schema::messages;
use crate::schema::users;
use diesel::prelude::*;

/* User Model */
#[derive(Queryable)]
pub struct User {
    pub user_id: i32,
    pub username: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub user_id: &'a i32,
    pub username: &'a String,
}

/* Message Model */
#[derive(Queryable)]
pub struct Message {
    pub message_id: i32,
    pub text: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage<'a> {
    pub message_id: &'a i32,
    pub text: &'a String,
    pub user_id: &'a i32,
}
