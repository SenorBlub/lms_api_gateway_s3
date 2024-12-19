use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData, errors::Error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,  // Subject (usually user ID)
    Jti: String, // Unique Token Id for revocation
    exp: usize,   // Expiry time (as UTC timestamp)
}

const SECRET: &[u8] = b"very-secret-key";

pub fn validate_jwt(token: &str) -> Result<String, Error> {
    let token_data: TokenData<Claims> = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_data.claims.sub)
}
