use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData, errors::ErrorKind};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct JwtConfig {
    #[serde(rename = "alg")]
    alg: String,
    #[serde(rename = "typ")]
    typ: String,
    #[serde(rename = "sub")]
    sub: String,
    #[serde(rename = "jti")]
    jti: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "iat")]
    iat: usize,
    #[serde(rename = "exp")]
    exp: usize,
    #[serde(rename = "secret")]
    secret: String,
    #[serde(rename = "iss")]
    issuer: String,
    #[serde(rename = "aud")]
    audience: String,
}

/// Fetch the secret key from environment variables
fn get_secret_key() -> Result<String, Error> {
    env::var("JWT_SECRET_KEY").map_err(|_| ErrorKind::InvalidSignature)
}

pub fn validate_jwt(token: &str) -> Result<String, Error> {
    let secret = get_secret_key()?;
    
    let token_data: TokenData<JwtConfig> = decode::<JwtConfig>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    let current_timestamp = Utc::now().timestamp() as usize;
    if token_data.claims.exp < current_timestamp {
        return Err(ErrorKind::ExpiredSignature);
    }

    Ok(token_data.claims.sub)
}
