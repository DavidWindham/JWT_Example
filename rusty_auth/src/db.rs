use crate::auth::RefreshToken;
use crate::schema::{refresh_tokens, users};
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

    let _user_insert_usize = match insert_call {
        Ok(user_insert_usize) => user_insert_usize,
        Err(e) => {
            return Err(format!(
                "Couldn't register user, user was likely already found: {}",
                e
            ));
        }
    };

    // if user_insert_usize == 1 {
    //     // Not sure what this usize is actually representing, it's not the number of users inserted
    // } else {
    //     return Ok(user_db.to_user());
    // }

    return Ok(user_db.to_user());
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

    let user_db = match user_call {
        Ok(user_db) => user_db,
        Err(e) => {
            eprintln!("Error getting user from DB: {}", e);
            return Err(format!("Error getting user from DB: {}", e));
        }
    };

    println!("User has been loaded");
    let password_validation_call = user_db.validate_password(passed_password_to_check);
    let validation_bool = match password_validation_call {
        Ok(validation_bool) => validation_bool,
        Err(e) => {
            eprintln!("Error calling validation: {}", e);
            return Err(format!("Error trying to validate password: {}", e));
        }
    };

    println!("Validation was: {}", validation_bool);
    if !validation_bool {
        return Err("Password was invalid".to_string());
    }

    return Ok(user_db.to_user());
}

pub fn get_user_from_uuid(
    conn: &mut DBPooledConnection,
    user_uuid: uuid::Uuid,
) -> Result<User, String> {
    use crate::schema::users::dsl::*;
    let user_call = users.filter(id.eq(user_uuid)).first::<UserDB>(conn);
    let user_db = match user_call {
        Ok(user_db) => user_db,
        Err(e) => {
            eprintln!("Error getting user from DB: {}", e);
            return Err(format!("Error getting user from DB: {}", e));
        }
    };
    Ok(user_db.to_user())
}

// pub fn _save_refresh_token_to_db(
//     conn: &mut DBPooledConnection,
//     refresh_token: RefreshToken,
// ) -> Result<RefreshToken, String> {
//     let refresh_token_db = refresh_token.to_db();
//     let insert_call: Result<usize, diesel::result::Error> =
//         diesel::insert_into(refresh_tokens::table)
//             .values(&refresh_token_db)
//             .execute(conn);

//     let refresh_insert_usize = match insert_call {
//         Ok(refresh_insert_usize) => refresh_insert_usize,
//         Err(e) => {
//             eprintln!("Error inserting token into DB: {}", e);
//             return Err("Couldn't insert new refresh token into DB".to_string());
//         }
//     };

//     if refresh_insert_usize != 1 {
//         eprintln!("Refresh token was already found in DB, this shouldn't happen");
//         return Err("Refresh token was already found in DB, this shouldn't happen".to_string());
//     }

//     return Ok(refresh_token_db.to_refresh_token());
// }

pub fn _blacklist_refresh_token(
    conn: &mut DBPooledConnection,
    refresh_token: RefreshToken,
) -> Result<String, String> {
    Ok("".to_string())
}

pub fn _check_refresh_token_against_db(
    _conn: &mut DBPooledConnection,
    _passed_refresh_token: String,
) -> Result<String, String> {
    return Ok("".to_string());
}
