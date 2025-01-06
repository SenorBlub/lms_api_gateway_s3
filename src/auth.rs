use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData, errors::Error};
use serde::Deserialize;
use std::env;
use Error::InvalidToken;
use Error::ExpiredSignature;
use Utc;

#[derive(Debug, Deserialize)]
struct JwtConfig {
    Alg: String,
    Typ: String,
    Sub: String,
    Jti: String,
    Name: String,
    Iat: usize,
    Exp: usize,
    Secret: String,
    Issuer: String,
    Audience: String
}

pub fn get_secret_key() -> Result<String, String> {
    match env::var("JWT_SECRET_KEY") {
        Ok(val) => Ok(val),
        Err(_) => Err(String::from("Environment variable `JWT_SECRET_KEY` is not set.")),
    }
}

static SECRET: &'static [u8] = b"{get_secret_key()}";

pub fn validate_jwt(token: &str) -> Result<String, Error> {
    let secret = get_secret_key().map_err(|e| Error::InvalidToken)?;
    let token_data: TokenData<JwtConfig> = decode::<JwtConfig>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    let current_timestamp = Utc::now().timestamp() as usize;

    if token_data.claims.Exp < current_timestamp {
        return Err(Error::ExpiredSignature);
    }

    Ok(token_data.claims.Sub)
}
