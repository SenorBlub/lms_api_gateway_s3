use chrono::Utc;
use jsonwebtoken::{decode, errors::{Error, ErrorKind}, Algorithm, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct JwtConfig {
    #[serde(rename = "sub")]
    sub: String,
    #[serde(rename = "jti")]
    jti: String,
    #[serde(rename = "iat")]
    iat: i64, 
    #[serde(rename = "nbf")]
    nbf: i64, 
    #[serde(rename = "exp")]
    exp: i64, 
    #[serde(rename = "iss")]
    iss: String,
    #[serde(rename = "aud")]
    aud: String,
}

fn get_secret_key() -> Result<String, Error> {
    env::var("JWT_SECRET_KEY").map_err(|_| ErrorKind::InvalidToken.into())
}

pub fn validate_jwt(token: &str) -> Result<String, Error> {
    let secret = get_secret_key()?;
    let secret_bytes = secret.as_bytes();
    println!("Secret Key: {}", secret);

    let decodable_token = token.to_string();
    println!("Token: {}", decodable_token);

    let token_data: TokenData<JwtConfig> = decode::<JwtConfig>(
        &decodable_token,
        &DecodingKey::from_secret(secret_bytes),
        &Validation::new(Algorithm::HS256),
    ).map_err(|err| {
        println!("JWT Decode Error: {:?}", err);
        err
    })?;

    println!("Decoded Token: {:?}", token_data);

    let current_timestamp = Utc::now().timestamp();
    if token_data.claims.exp < current_timestamp {
        println!("Token has expired.");
        return Err(ErrorKind::ExpiredSignature.into());
    } else if token_data.claims.nbf > current_timestamp {
        println!("Token is not yet valid.");
        return Err(ErrorKind::ImmatureSignature.into());
    }

    println!("Token is valid.");
    Ok(token_data.claims.sub)
}
