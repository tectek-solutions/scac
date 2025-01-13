use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use chrono::{Utc, Duration};
use std::env;
use actix_web::web;
use redis::Commands;

use cache;

pub fn signing_jwt(cache: &web::Data<cache::Cache>, user_id: i32) -> Result<String, String> {
    let mut cache_connection = cache.get_connection();

    let jwt_secret = env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set".to_string())?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_ref())
        .map_err(|_| "HMAC creation failed".to_string())?;
    
    let mut claims = BTreeMap::new();
    claims.insert("id".to_string(), user_id.to_string());
    claims.insert(
        "expiration".to_string(),
        (Utc::now() + Duration::days(1)).timestamp().to_string(),
    );

    let token = claims
        .sign_with_key(&key)
        .map_err(|_| "Failed to sign claims".to_string())?;

    // Store the token in Redis
    let _: () = cache_connection
        .set_ex(format!("token:{}", user_id), &token, 86400)
        .map_err(|_| "Failed to store token in Redis".to_string())?;

    Ok(token)
}


pub fn verify_jwt(cache: &web::Data<cache::Cache>, token: &str) -> bool {
    let mut cache_connection = cache.get_connection();

    let jwt_secret = match env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => return false,
    };

    let key: Hmac<Sha256> = match Hmac::new_from_slice(jwt_secret.as_ref()) {
        Ok(k) => k,
        Err(_) => return false,
    };

    let claims: BTreeMap<String, String> = match token.verify_with_key(&key) {
        Ok(c) => c,
        Err(_) => return false,
    };

    if let Some(expiration) = claims.get("expiration") {
        if let Ok(expiration_ts) = expiration.parse::<i64>() {
            if expiration_ts < Utc::now().timestamp() {
                return false; // Token has expired
            }
        } else {
            return false; // Invalid expiration timestamp
        }
    }

    // Check if token exists in Redis
    let redis_key = format!("token:{}", claims.get("id").unwrap_or(&"".to_string()));
    match cache_connection.exists(&redis_key) {
        Ok(true) => true,
        _ => false,
    }
}

pub fn get_user_id_by_jwt(cache: &web::Data<cache::Cache>, token: &str) -> Result<Option<i32>, String> {
    let mut cache_connection = cache.get_connection();

    let jwt_secret = env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set".to_string())?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_ref())
        .map_err(|_| "HMAC creation failed".to_string())?;

    let claims: BTreeMap<String, String> = token
        .verify_with_key(&key)
        .map_err(|_| "Failed to verify token".to_string())?;

    // Check if token exists in Redis
    let redis_key = format!("token:{}", claims.get("id").unwrap_or(&"".to_string()));
    if !cache_connection.exists(&redis_key).map_err(|_| "Redis check failed".to_string())? {
        return Err("Token not found in Redis".to_string());
    }

    // Extract user ID
    match claims.get("id") {
        Some(id) => match id.parse::<i32>() {
            Ok(id) => Ok(Some(id)),
            Err(_) => Err("Failed to parse user ID".to_string()),
        },
        None => Ok(None),
    }
}

pub fn delete_jwt(cache: &web::Data<cache::Cache>, token: &str) -> Result<(), String> {
    let mut cache_connection = cache.get_connection();

    let jwt_secret = env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set".to_string())?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_ref())
        .map_err(|_| "HMAC creation failed".to_string())?;

    let claims: BTreeMap<String, String> = token
        .verify_with_key(&key)
        .map_err(|_| "Failed to verify token".to_string())?;

    // Check if token exists in Redis
    let redis_key = format!("token:{}", claims.get("id").unwrap_or(&"".to_string()));
    if !cache_connection.exists(&redis_key).map_err(|_| "Redis check failed".to_string())? {
        return Err("Token not found in Redis".to_string());
    }

    // Delete token from Redis
    match cache_connection.del::<_, ()>(&redis_key) {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to delete token from Redis".to_string()),
    }
}
