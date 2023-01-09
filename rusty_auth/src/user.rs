use crate::auth::{generate_access_token, generate_refresh_token};
use crate::DBPool;
use crate::{
    consts::{APPLICATION_JSON, CONNECTION_POOL_ERROR},
    db::{login_user_against_db, register_new_user_to_db},
    schema::{refresh_tokens, users},
};
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
        if let Ok(password_hash) = password_hash_call {
            println!(
                "Password hash for {} is  {}",
                password,
                password_hash.to_string()
            );
            Ok(Self {
                id: uuid::Uuid::new_v4(),
                username: username.to_string(),
                password_hash: password_hash.to_string(),
                created_at: Utc::now(),
            })
        } else {
            eprintln!("Error getting password hash");
            Err("Error generating password hash".to_string())
        }
    }

    fn generate_password_hash(password: &str) -> Result<HashParts, BcryptError> {
        let salt = "password_salt";
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

    pub fn generate_access_token(&self) -> String {
        let access_token = generate_access_token(self.username.clone());
        println!(
            "Access token has been generated: {}",
            access_token.clone().unwrap()
        );
        // let access_token = is_access_token_valid(&access_token.unwrap());
        return access_token.unwrap().to_string();
    }
    pub fn generate_refresh_token(&self) -> String {
        let refresh_token = generate_refresh_token(self.username.clone());
        println!(
            "Refresh token has been generated: {}",
            refresh_token.clone().unwrap()
        );

        return refresh_token.unwrap().to_string();
    }
}

#[table_name = "users"]
#[derive(Queryable, Insertable)]
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
        if let Some(username) = &self.username {
            if let Some(password) = &self.password {
                return User::new(username, password);
            }
        }
        return Err("Error getting username or passord".to_string());
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl LoginUserRequest {}

#[post("/register")]
pub async fn register_new_user(
    register_user_request: Json<RegisterUserRequest>,
    pool: Data<DBPool>,
) -> HttpResponse {
    if let Ok(user_from_request) = register_user_request.to_user() {
        let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
        let new_user_call =
            web::block(move || register_new_user_to_db(&mut conn, user_from_request)).await;
        return match new_user_call {
            Ok(new_user_insert) => match new_user_insert {
                Ok(new_user) => {
                    println!("New user success {} - {}", new_user.username, new_user.id);
                    HttpResponse::Accepted()
                        .content_type(APPLICATION_JSON)
                        .json({})
                }
                Err(e) => {
                    eprintln!("Error inserting new user: {}", e);
                    HttpResponse::Conflict()
                        .content_type(APPLICATION_JSON)
                        .json({})
                }
            },
            Err(e) => HttpResponse::NotAcceptable()
                .content_type(APPLICATION_JSON)
                .json({}),
        };
    }
    return HttpResponse::NotAcceptable()
        .content_type(APPLICATION_JSON)
        .json({});
}

#[post("/login")]
pub async fn login_user(
    login_user_request: Json<LoginUserRequest>,
    pool: Data<DBPool>,
) -> HttpResponse {
    if let Some(username) = login_user_request.username.clone() {
        if let Some(password) = login_user_request.password.clone() {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            let login_web_blocked_call =
                web::block(move || login_user_against_db(&mut conn, username, password)).await;
            match login_web_blocked_call {
                Ok(login_call) => match login_call {
                    Ok(logged_in_user) => {
                        println!("Login success");
                        // Here, I need to generate auth token and refresh token
                        let access_token = logged_in_user.generate_access_token();
                        let refresh_token = logged_in_user.generate_refresh_token();
                        return HttpResponse::Accepted()
                            .content_type(APPLICATION_JSON)
                            .json(json!({ "access_token": access_token, "refresh_token": refresh_token }));
                    }
                    Err(e) => {
                        eprintln!("Error with login call: {}", e);
                    }
                },
                Err(e) => {
                    eprintln!("Error with the web block: {}", e);
                }
            }
        }
    }

    return HttpResponse::NotAcceptable()
        .content_type(APPLICATION_JSON)
        .json({});
}
