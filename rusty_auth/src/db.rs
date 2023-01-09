use crate::schema::users;
use diesel::pg::PgConnection;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::user::{User, UserDB};
use crate::DBPooledConnection;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn register_new_user_to_db(
    conn: &mut DBPooledConnection,
    new_user: User,
) -> Result<User, String> {
    let user_db = new_user.to_user_db();
    let insert_call: Result<usize, diesel::result::Error> = diesel::insert_into(users::table)
        .values(&user_db)
        .execute(conn);
    match insert_call {
        Ok(user_usize) => {
            if user_usize == 1 {
                return Ok(user_db.to_user());
            } else {
                return Ok(user_db.to_user());
            }
        }
        Err(e) => {
            return Err("Lol".to_string());
        }
    }
}

pub fn login_user_against_db(
    conn: &mut DBPooledConnection,
    passed_username: String,
    passed_password_to_check: String,
) -> Result<User, String> {
    use crate::schema::users::dsl::*;
    let user_call = users
        .filter(username.eq(passed_username))
        .first::<UserDB>(conn);
    match user_call {
        Ok(user) => {
            println!("User has been loaded");
            let password_validation_call = user.validate_password(passed_password_to_check);
            match password_validation_call {
                Ok(validation_bool) => {
                    println!("Validation was: {}", validation_bool);
                    if validation_bool {
                        return Ok(user.to_user());
                    } else {
                        return Err("Password was invalid".to_string());
                    }
                }
                Err(e) => {
                    eprintln!("Error calling validation: {}", e);
                    return Err(format!("Error trying to validate password: {}", e));
                }
            }
        }
        Err(e) => {
            eprintln!("Error getting user: {}", e);
            return Err(format!(
                "Error getting user, no user with the given username: {}",
                e
            ));
        }
    }
}

// pub fn save_refresh_token_to_db(
//     conn: &mut DBPooledConnection,
//     refresh_token: String,
// ) -> Result<String, String> {
//     return Ok("".to_string());
// }

// pub fn check_refresh_token_against_db(
//     conn: &mut DBPooledConnection,
//     passed_refresh_token: String,
// ) -> Result<String, String> {
//     return Ok("".to_string());
// }
