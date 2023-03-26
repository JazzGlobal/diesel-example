use diesel::prelude::*;
use DieselPractice::models::{NewUser, User};
use DieselPractice::schema::users::dsl::users;
use DieselPractice::{create_user, establish_connection};

fn main() {
    let connection = &mut establish_connection();
    let results = users
        .limit(5)
        .load::<User>(connection)
        .expect("Error loading posts");

    println!("Displaying {} Users", results.len());
    for user in results {
        println!("{}", user.username);
    }
}
