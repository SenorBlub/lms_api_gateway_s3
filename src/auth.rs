use chrono::Utc;
use jsonwebtoken::{decode, errors::{Error, ErrorKind}, Algorithm, DecodingKey, TokenData, Validation};
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
    #[serde(rename = "iat")]
    iat: usize,
    #[serde(rename = "nbf")]
    nbf: usize,
    #[serde(rename = "exp")]
    exp: usize,
    #[serde(rename = "secret")]
    secret: String,
    #[serde(rename = "iss")]
    issuer: String,
    #[serde(rename = "aud")]
    audience: String,
}

fn get_secret_key() -> Result<String, Error> {
    env::var("JWT_SECRET_KEY").map_err(|_| ErrorKind::InvalidToken.into())
}

pub fn validate_jwt(token: &str) -> Result<String, Error> {
    let secret = get_secret_key()?;
    println!(secret);

    let token_data: TokenData<JwtConfig> = decode::<JwtConfig>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    print!("{}", token_data);

    let current_timestamp = Utc::now().timestamp() as usize;
    if token_data.claims.exp < current_timestamp {
        return Err(ErrorKind::ExpiredSignature.into());
    }else if token_data.claims.nbf > current_timestamp {
        return Err(ErrorKind::ImmatureSignature.into());
    }

    Ok(token_data.claims.sub)
}
