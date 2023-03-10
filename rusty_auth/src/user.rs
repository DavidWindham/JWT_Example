use std::env;

use crate::auth::{generate_access_token, generate_refresh_token};
use crate::{
    consts::{APPLICATION_JSON, CONNECTION_POOL_ERROR},
    db::{login_user_against_db, register_new_user_to_db},
    schema::users,
};
use crate::{DBPool, DBPooledConnection};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use bcrypt::{self, BcryptError, HashParts};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: &str, password: &str) -> Result<Self, String> {
        let password_hash_call = User::generate_password_hash(password);
        let password_hash = match password_hash_call {
            Ok(password_hash) => password_hash,
            Err(e) => {
                eprintln!("Error getting password hash: {}", e);
                return Err(format!("Error generating password hash: {}", e));
            }
        };

        Ok(Self {
            id: uuid::Uuid::new_v4(),
            username: username.to_string(),
            password_hash: password_hash.to_string(),
            created_at: Utc::now(),
        })
    }

    fn generate_password_hash(password: &str) -> Result<HashParts, BcryptError> {
        let salt = env::var("PASSWORD_SALT").expect("PASSWORD_SALT must be set");
        let mut salt_arr = [0; 16];
        salt_arr[..salt.len()].copy_from_slice(salt.as_bytes());
        return bcrypt::hash_with_salt(password, 10, salt_arr);
    }

    pub fn to_user_db(&self) -> UserDB {
        UserDB {
            id: uuid::Uuid::new_v4(),
            username: self.username.clone(),
            password_hash: self.password_hash.clone(),
            created_at: Utc::now().naive_utc(),
        }
    }

    pub fn generate_access_token(&self) -> Result<String, String> {
        let access_token = match generate_access_token(self.username.clone()) {
            Ok(access_token) => access_token,
            Err(e) => {
                return Err(format!("Error getting access token: {}", e));
            }
        };
        println!("Access token has been generated: {}", access_token.clone());
        return Ok(access_token.to_string());
    }
    pub fn generate_refresh_token(&self) -> Result<String, String> {
        let refresh_token = match generate_refresh_token(self.username.clone()) {
            Ok(refresh_token) => refresh_token,
            Err(e) => {
                return Err(format!("Error getting refresh token: {}", e));
            }
        };
        println!(
            "Refresh token has been generated: {}",
            refresh_token.clone()
        );

        return Ok(refresh_token.to_string());
    }
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct UserDB {
    pub id: uuid::Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

impl UserDB {
    pub fn to_user(&self) -> User {
        User {
            id: self.id.clone(),
            username: self.username.clone(),
            password_hash: self.password_hash.clone(),
            created_at: Utc.from_utc_datetime(&self.created_at),
        }
    }
    pub fn validate_password(&self, password_to_check: String) -> Result<bool, BcryptError> {
        return bcrypt::verify(password_to_check, &self.password_hash);
    }
}

// Request interfaces
#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl RegisterUserRequest {
    pub fn to_user(&self) -> Result<User, String> {
        let username = match &self.username {
            Some(username) => username,
            None => return Err("Error getting username".to_string()),
        };

        let password = match &self.password {
            Some(password) => password,
            None => return Err("Error getting password".to_string()),
        };

        return User::new(username, password);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl LoginUserRequest {
    pub fn validate_inputs(&self) -> Result<(), String> {
        match self.username.clone() {
            Some(_) => (),
            None => return Err("Missing username".to_string()),
        };

        match self.password.clone() {
            Some(_) => (),
            None => return Err("Missing password".to_string()),
        };

        Ok(())
    }
    pub async fn to_user(&self, mut conn: DBPooledConnection) -> Result<User, String> {
        let username = self.username.clone().unwrap();
        let password = self.password.clone().unwrap();

        let web_block_call =
            web::block(move || login_user_against_db(&mut conn, username, password)).await;

        let user_login_call = match web_block_call {
            Ok(user_login_call) => user_login_call,
            Err(e) => {
                eprintln!("Error with web block: {}", e);
                return Err(format!("Error fetching user: {}", e));
            }
        };

        let user = match user_login_call {
            Ok(user) => user,
            Err(e) => {
                eprintln!("Error getting user: {}", e);
                return Err(format!("Error getting user: {}", e));
            }
        };

        return Ok(user);
    }
}

#[post("/register")]
pub async fn register_new_user(
    register_user_request: Json<RegisterUserRequest>,
    pool: Data<DBPool>,
) -> HttpResponse {
    let user_from_request = match register_user_request.to_user() {
        Ok(user_from_request) => user_from_request,
        Err(e) => {
            return HttpResponse::NotAcceptable()
                .content_type(APPLICATION_JSON)
                .json(json!({ "status": e }))
        }
    };

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let new_user_call =
        web::block(move || register_new_user_to_db(&mut conn, user_from_request)).await;

    let new_user_insert = match new_user_call {
        Ok(new_user_insert) => new_user_insert,
        Err(e) => {
            return HttpResponse::NotAcceptable()
                .content_type(APPLICATION_JSON)
                .json(json!({ "status": format!("{}", e) }))
        }
    };

    let new_user = match new_user_insert {
        Ok(new_user) => new_user,
        Err(e) => {
            eprintln!("Error inserting new user");
            return HttpResponse::NotAcceptable()
                .content_type(APPLICATION_JSON)
                .json(json!({ "status": format!("{}", e) }));
        }
    };

    let access_token = match new_user.generate_access_token() {
        Ok(access_token) => access_token,
        Err(e) => {
            return HttpResponse::NotAcceptable()
                .content_type(APPLICATION_JSON)
                .json(json!({ "status": format!("{}", e) }));
        }
    };

    let refresh_token = match new_user.generate_refresh_token() {
        Ok(refresh_token) => refresh_token,
        Err(e) => {
            return HttpResponse::NotAcceptable()
                .content_type(APPLICATION_JSON)
                .json(json!({ "status": format!("{}", e) }))
        }
    };

    return HttpResponse::Accepted()
        .content_type(APPLICATION_JSON)
        .json(json!({ "access_token": access_token, "refresh_token": refresh_token }));
}

#[post("/login")]
pub async fn login_user(
    login_user_request: Json<LoginUserRequest>,
    pool: Data<DBPool>,
) -> HttpResponse {
    match login_user_request.validate_inputs() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error checking username and password: {}", e);
            return HttpResponse::NotAcceptable()
                .content_type(APPLICATION_JSON)
                .json(json!({"status": "No username or password provided"}));
        }
    }

    let conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let logged_in_user = match login_user_request.to_user(conn).await {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Error getting user: {}", e);
            return HttpResponse::NotAcceptable()
                .content_type(APPLICATION_JSON)
                .json(json!({ "status": format!("Failed to login: {}", e) }));
        }
    };

    // Here, I need to generate auth token and refresh token
    let access_token = match logged_in_user.generate_access_token() {
        Ok(access_token) => access_token,
        Err(e) => {
            return HttpResponse::NotAcceptable()
                .content_type(APPLICATION_JSON)
                .json(json!({ "status": format!("{}", e) }));
        }
    };

    let refresh_token = match logged_in_user.generate_refresh_token() {
        Ok(refresh_token) => refresh_token,
        Err(e) => {
            return HttpResponse::NotAcceptable()
                .content_type(APPLICATION_JSON)
                .json(json!({ "status": format!("{}", e) }))
        }
    };

    return HttpResponse::Accepted()
        .content_type(APPLICATION_JSON)
        .json(json!({ "access_token": access_token, "refresh_token": refresh_token }));
}
