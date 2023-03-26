use self::models::{NewUser, User};
use crate::models::{Message, NewMessage};
use crate::schema::messages::dsl::messages;
use crate::schema::messages::message_id;
use diesel::associations::HasTable;
use diesel::{Connection, PgConnection, RunQueryDsl};
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut PgConnection, user_id: &i32, username: &String) -> User {
    use crate::schema::users;
    let new_user = NewUser {
        user_id,
        username: &String::from("waymer"),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

/// Inserts a Message into the database and returns it.
pub fn create_message(
    conn: &mut PgConnection,
    msg_id: &i32,
    user: &User,
    text: &String,
) -> Message {
    use crate::schema::messages;
    let new_message = NewMessage {
        message_id: msg_id,
        user_id: &user.user_id,
        text,
    };
    diesel::insert_into(messages::table)
        .values(&new_message)
        .get_result(conn)
        .expect("Error saving new message")
}
