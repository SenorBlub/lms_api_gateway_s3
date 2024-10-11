use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData, errors::Error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,  // Subject (usually user ID)
    exp: usize,   // Expiry time (as UTC timestamp)
}

const SECRET: &[u8] = b"your-very-secret-key";

pub fn validate_jwt(token: &str) -> Result<String, Error> {
    let token_data: TokenData<Claims> = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_data.claims.sub)
}
