use crate::user::User;
use crate::DBPool;
use crate::{consts::APPLICATION_JSON, schema::refresh_tokens};
use actix_web::HttpRequest;
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::Queryable;
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Claims, Header, RegisteredClaims, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::Sha384;
use std::collections::BTreeMap;
use std::env;

// pub struct RefreshToken {
//     pub id: uuid::Uuid,
//     pub user_id: uuid::Uuid,
//     pub token: String,
//     pub valid_until: NaiveDateTime,
// }

// impl RefreshToken {
//     pub fn new(user: User, token: String) -> Self {
//         Self {
//             id: uuid::Uuid::new_v4(),
//             user_id: user.id,
//             token: token,
//             valid_until: (Utc::now() + Duration::minutes(2)).naive_utc(),
//         }
//     }
// }

// #[table_name = "refresh_tokens"]
// #[derive(Queryable, Insertable)]
// pub struct RefreshTokenDB {
//     pub id: uuid::Uuid,
//     pub user_id: uuid::Uuid,
//     pub token: String,
//     pub valid_until: NaiveDateTime,
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: Option<String>,
}

impl RefreshTokenRequest {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenRequest {
    pub access_token: Option<String>,
}

impl AccessTokenRequest {}

#[post("/verify_token")]
pub async fn verify_token(request: HttpRequest) -> HttpResponse {
    let token_first_part = request.headers().get("access_token");

    match token_first_part {
        Some(token_unwrapped) => {
            let token_to_str_call = token_unwrapped.to_str();
            match token_to_str_call {
                Ok(token) => {
                    let validate_call = is_access_token_valid(token);
                    match validate_call {
                        Ok(status) => {
                            return HttpResponse::Accepted()
                                .content_type(APPLICATION_JSON)
                                .json(json!({ "status": status }));
                        }
                        Err(e) => {
                            eprintln!("Error validating call: {}", e);
                            return HttpResponse::Unauthorized()
                                .content_type(APPLICATION_JSON)
                                .json(json!({ "status": e.clone() }));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error ToString: {:?}", e);
                    return HttpResponse::Unauthorized()
                        .content_type(APPLICATION_JSON)
                        .json(json!({ "status": "Failed converting token to string" }));
                }
            }
        }
        None => HttpResponse::Unauthorized()
            .content_type(APPLICATION_JSON)
            .json(json!({ "status": "No token in header" })),
    }
}

#[post("/refresh_token")]
pub async fn refresh_token(
    refresh_token_request: Json<RefreshTokenRequest>,
    pool: Data<DBPool>,
) -> HttpResponse {
    let refresh_validation_call = is_refresh_token_valid(
        refresh_token_request
            .refresh_token
            .clone()
            .unwrap()
            .as_str(),
    );

    match refresh_validation_call {
        Ok(username) => {
            println!("Refresh token is valid, username: {}", username);
            let new_access_token_call = generate_access_token(username);
            match new_access_token_call {
                Ok(new_access_token) => {
                    return HttpResponse::Accepted()
                        .content_type(APPLICATION_JSON)
                        .json(json!({ "access_token": new_access_token }));
                }
                Err(_) => {
                    return HttpResponse::Unauthorized()
                        .content_type(APPLICATION_JSON)
                        .json({});
                }
            }
        }
        Err(e) => {
            eprintln!("Error validating refresh token, token is invalid: {}", e);
            return HttpResponse::Unauthorized()
                .content_type(APPLICATION_JSON)
                .json(json!({ "status": e }));
        }
    }
}

pub fn generate_access_token(username: String) -> Result<String, ()> {
    let access_token_secret =
        env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set");

    let access_token_seconds = env::var("ACCESS_TOKEN_EXPIRE_SECONDS")
        .expect("DATABASE_URL must be set")
        .parse::<i64>()
        .unwrap();

    let key: Hmac<Sha384> = Hmac::new_from_slice(access_token_secret.as_bytes()).unwrap();
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };

    let iat_epoch = Utc::now().naive_utc().timestamp();
    let exp_epoch = (Utc::now() + Duration::seconds(access_token_seconds))
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

    let token = Token::new(header, claims).sign_with_key(&key).unwrap();

    return Ok(token.as_str().to_string());
}

pub fn is_access_token_valid(token_str: &str) -> Result<String, String> {
    let access_token_secret =
        env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set");

    let key: Hmac<Sha384> = Hmac::new_from_slice(access_token_secret.as_bytes()).unwrap();
    let token_call: Result<Token<Header, BTreeMap<String, Value>, _>, _> =
        token_str.verify_with_key(&key);
    match token_call {
        Ok(token) => {
            let header = token.header();
            let claims = token.claims();
            assert_eq!(header.algorithm, AlgorithmType::Hs384);

            let exp_naive_datetime =
                NaiveDateTime::from_timestamp(claims["exp"].as_i64().unwrap(), 0);

            if exp_naive_datetime > Utc::now().naive_utc() {
                println!("Token is still valid");
                return Ok("Token is valid".to_string());
            }

            println!("Token has expired");
            return Err("Token expired".to_string());
        }
        Err(e) => {
            eprintln!("Error verifying token, token is invalid: {}", e);
            return Err("Token could not be verified, it's likely been tampered with".to_string());
        }
    }
}

pub fn generate_refresh_token(username: String) -> Result<String, ()> {
    let refresh_token_secret =
        env::var("REFRESH_TOKEN_SECRET").expect("REFRESH_TOKEN_SECRET must be set");
    let refresh_token_seconds = env::var("REFRESH_TOKEN_EXPIRE_SECONDS")
        .expect("DATABASE_URL must be set")
        .parse::<i64>()
        .unwrap();

    let key: Hmac<Sha384> = Hmac::new_from_slice(refresh_token_secret.as_bytes()).unwrap();
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };

    let iat_epoch = Utc::now().naive_utc().timestamp();
    let exp_epoch = (Utc::now() + Duration::seconds(refresh_token_seconds))
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

    let token = Token::new(header, claims).sign_with_key(&key).unwrap();

    return Ok(token.as_str().to_string());
}

pub fn is_refresh_token_valid(token_str: &str) -> Result<String, String> {
    let refresh_token_secret =
        env::var("REFRESH_TOKEN_SECRET").expect("REFRESH_TOKEN_SECRET must be set");

    let key: Hmac<Sha384> = Hmac::new_from_slice(refresh_token_secret.as_bytes()).unwrap();
    let token_call: Result<Token<Header, BTreeMap<String, Value>, _>, _> =
        token_str.verify_with_key(&key);
    match token_call {
        Ok(token) => {
            let header = token.header();
            let claims = token.claims();
            assert_eq!(header.algorithm, AlgorithmType::Hs384);

            let exp_naive_datetime =
                NaiveDateTime::from_timestamp(claims["exp"].as_i64().unwrap(), 0);

            if exp_naive_datetime < Utc::now().naive_utc() {
                println!("Token has expired");
                return Err("Token expired".to_string());
            } else {
                println!("Token is still valid");
                return Ok(claims["username"].as_str().unwrap().to_string());
            }
        }
        Err(e) => {
            eprintln!("Error verifying token, token is invalid: {}", e);
            return Err("Token could not be verified, it's likely been tampered with".to_string());
        }
    }
}
