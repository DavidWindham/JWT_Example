// use crate::consts::CONNECTION_POOL_ERROR;
use crate::db::get_user_from_uuid;
use crate::user::User;
use crate::{consts::APPLICATION_JSON, schema::refresh_tokens};
use crate::{DBPool, DBPooledConnection};
use actix_web::HttpRequest;
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::Queryable;
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Claims, Header, RegisteredClaims, SignWithKey, Token, VerifyWithKey};
use rusty_auth::auth_errors::access_token_errors::TokenError;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::Sha384;
use std::collections::BTreeMap;
use std::env;

pub struct RefreshToken {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub username: String,
    pub token: String,
    pub valid_until: NaiveDateTime,
}

impl RefreshToken {
    pub fn new(user: User, token: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            user_id: user.id,
            username: user.username,
            token: token,
            valid_until: (Utc::now() + Duration::minutes(2)).naive_utc(),
        }
    }

    pub fn to_db(&self) -> RefreshTokenDB {
        RefreshTokenDB {
            id: uuid::Uuid::new_v4(),
            user_id: self.user_id,
            token: self.token.clone(),
            valid_until: self.valid_until,
        }
    }

    // pub fn generate_new_access_token(&self) -> Result<String, String> {
    //     // generate_access_token(username)
    // }
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = refresh_tokens)]
pub struct RefreshTokenDB {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub token: String,
    pub valid_until: NaiveDateTime,
}

impl RefreshTokenDB {
    pub fn to_refresh_token(&self, conn: &mut DBPooledConnection) -> Result<RefreshToken, String> {
        let user = match get_user_from_uuid(conn, self.user_id.clone()) {
            Ok(user) => user,
            Err(e) => {
                eprintln!("Error getting assoc'd user");
                return Err(format!("Error getting user for refresh token: {}", e));
            }
        };
        Ok(RefreshToken {
            id: self.id,
            user_id: self.user_id,
            username: user.username,
            token: self.token.clone(),
            valid_until: self.valid_until,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: Option<String>,
}

impl RefreshTokenRequest {
    pub fn _get_refresh_token(&self) {}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenRequest {
    pub access_token: Option<String>,
}

impl AccessTokenRequest {}

#[post("/verify_token")]
pub async fn verify_token(request: HttpRequest) -> HttpResponse {
    let token_unwrapped = match request.headers().get("access_token") {
        Some(token_first_part) => token_first_part,
        None => {
            return HttpResponse::Unauthorized()
                .content_type(APPLICATION_JSON)
                .json(json!({ "status": "No token in header" }))
        }
    };

    let token = match token_unwrapped.to_str() {
        Ok(token_as_str) => token_as_str,
        Err(e) => {
            eprintln!("Error ToString: {:?}", e);
            return HttpResponse::Unauthorized()
                .content_type(APPLICATION_JSON)
                .json(json!({ "status": "Failed converting token to string" }));
        }
    };

    let status = match is_access_token_valid(token) {
        Ok(status) => status,
        Err(e) => match e {
            TokenError::ExpiredToken => {
                return HttpResponse::Unauthorized()
                    .content_type(APPLICATION_JSON)
                    .json(json!({ "status": e.to_string() }));
            }
            _ => {
                return HttpResponse::NotAcceptable()
                    .content_type(APPLICATION_JSON)
                    .json(json!({ "status": e.to_string() }));
            }
        },
    };

    return HttpResponse::Accepted()
        .content_type(APPLICATION_JSON)
        .json(json!({ "status": status }));
}

#[post("/refresh_token")]
pub async fn refresh_token(
    refresh_token_request: Json<RefreshTokenRequest>,
    _pool: Data<DBPool>,
) -> HttpResponse {
    let passed_refresh_token_string = match refresh_token_request.refresh_token.clone() {
        Some(refresh_token) => refresh_token,
        None => {
            return HttpResponse::Unauthorized()
                .content_type(APPLICATION_JSON)
                .json(json!({ "status": "No token in header" }))
        }
    };

    let username = match is_refresh_token_valid(&passed_refresh_token_string.as_str()) {
        Ok(username) => username,
        Err(e) => {
            return HttpResponse::Unauthorized()
                .content_type(APPLICATION_JSON)
                .json(json!({
                    "status": format!("Refresh Token was invalid: {}", e.to_string())
                }));
        }
    };

    // Could check against the DB if needed
    // let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    // let login_web_blocked_call =
    // web::block(move || login_user_against_db(&mut conn, username, password)).await;
    // RefreshTokenRequest.to_refresh_token()

    // TODO: Blacklist old refresh token before generating  new tokens
    // TODO: Add checks during this blacklist, listed below
    //      already blacklisted - token is being used by 2 different sessions, token hijacked, or localstorage sync issue
    //      perhaps some sort of IP hashing
    //      or fingerprinting hashing, validate against this

    let new_access_token = match generate_access_token(username.clone()) {
        Ok(new_access_token) => new_access_token,
        Err(e) => {
            return HttpResponse::Unauthorized()
                .content_type(APPLICATION_JSON)
                .json(json!({
                    "status": format!("Error getting new access token: {}", e)
                }));
        }
    };

    let new_refresh_token = match generate_refresh_token(username) {
        Ok(new_refresh_token) => new_refresh_token,
        Err(e) => {
            return HttpResponse::Unauthorized()
                .content_type(APPLICATION_JSON)
                .json(json!({
                    "status": format!("Error getting new refresh token: {}", e)
                }));
        }
    };

    return HttpResponse::Accepted()
        .content_type(APPLICATION_JSON)
        .json(json!({ "access_token": new_access_token, "refresh_token": new_refresh_token }));
}

pub fn generate_access_token(username: String) -> Result<String, TokenError> {
    let access_token_secret =
        env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set");
    let access_token_seconds = env::var("ACCESS_TOKEN_EXPIRE_SECONDS")
        .expect("DATABASE_URL must be set")
        .parse::<i64>()
        .unwrap();

    generate_token(access_token_secret, access_token_seconds, username)
}

pub fn generate_refresh_token(username: String) -> Result<String, TokenError> {
    let refresh_token_secret =
        env::var("REFRESH_TOKEN_SECRET").expect("REFRESH_TOKEN_SECRET must be set");
    let refresh_token_seconds = env::var("REFRESH_TOKEN_EXPIRE_SECONDS")
        .expect("DATABASE_URL must be set")
        .parse::<i64>()
        .unwrap();

    generate_token(refresh_token_secret, refresh_token_seconds, username)
}

pub fn is_access_token_valid(token_str: &str) -> Result<String, TokenError> {
    let access_token_secret =
        env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set");

    validate_token(token_str, access_token_secret)
}

pub fn is_refresh_token_valid(token_str: &str) -> Result<String, TokenError> {
    let secret = env::var("REFRESH_TOKEN_SECRET").expect("REFRESH_TOKEN_SECRET must be set");

    validate_token(token_str, secret)
}

fn generate_token(secret: String, seconds: i64, username: String) -> Result<String, TokenError> {
    let key: Hmac<Sha384> = match Hmac::new_from_slice(secret.as_bytes()) {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Error getting hmac: {}", e);
            return Err(TokenError::UnableToGetHmacKey);
        }
    };
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };

    let iat_epoch = Utc::now().naive_utc().timestamp();
    let exp_epoch = (Utc::now() + Duration::seconds(seconds))
        .naive_utc()
        .timestamp();

    let registered_claims = RegisteredClaims {
        issuer: Some("RustyAuth".to_string()),
        expiration: Some(exp_epoch as u64),
        issued_at: Some(iat_epoch as u64),
        ..Default::default()
    };

    // Set registered claims
    let mut claims = Claims::new(registered_claims);

    // Insert additional claims here, this would be outsourced to add specific roles
    let mut additional_claims: BTreeMap<String, Value> = BTreeMap::new();
    additional_claims.insert("username".to_string(), json!(username));
    claims.private = additional_claims;

    let token = match Token::new(header, claims).sign_with_key(&key) {
        Ok(token) => token,
        Err(e) => {
            eprintln!("Error getting token signed with key: {}", e);
            return Err(TokenError::UnableToSign);
        }
    };

    // RefreshTokenDB::new()

    // let RefreshToken =
    return Ok(token.as_str().to_string());
}

fn validate_token(token_str: &str, secret: String) -> Result<String, TokenError> {
    let key: Hmac<Sha384> = match Hmac::new_from_slice(secret.as_bytes()) {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Error getting hmac: {}", e);
            return Err(TokenError::UnableToGetHmacKey);
        }
    };
    let token_call: Result<Token<Header, BTreeMap<String, Value>, _>, _> =
        token_str.verify_with_key(&key);
    let token = match token_call {
        Ok(token) => token,
        Err(e) => {
            eprintln!("Error verifying token, token is invalid: {}", e);
            return Err(TokenError::InvalidToken);
        }
    };

    let header = token.header();
    let claims = token.claims();
    if header.algorithm != AlgorithmType::Hs384 {
        return Err(TokenError::InvalidHeaderAlgorithm);
        // return Err("Header has the incorrect algorithm".to_string());
    }

    let expiry = match claims["exp"].as_i64() {
        Some(expiry) => expiry,
        None => {
            eprintln!("Error unwrapping expiry");
            return Err(TokenError::InvalidExpiry);
        }
    };

    let exp_naive_datetime = match NaiveDateTime::from_timestamp_opt(expiry, 0) {
        Some(exp) => exp,
        None => return Err(TokenError::InvalidExpiry),
    };

    if exp_naive_datetime < Utc::now().naive_utc() {
        println!("Token has expired");
        return Err(TokenError::ExpiredToken);
    }

    let username = match claims["username"].as_str() {
        Some(username) => username,
        None => return Err(TokenError::MissingBodyKey),
    };

    return Ok(username.to_string());
}
