use dotenvy::dotenv;
use std::env;
use diesel::{Connection, PgConnection, RunQueryDsl};
use self::models::{NewUser, User};
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut PgConnection, user_id: &i32, username: &String) -> User {
    use crate::schema::users;
    let new_user = NewUser { user_id: &2, username: &String::from("waymer") };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}