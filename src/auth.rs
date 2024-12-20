use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData, errors::Error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,  // Subject (usually user ID)
    Jti: String, // Unique Token Id for revocation
    exp: usize,   // Expiry time (as UTC timestamp)
}

pub fn get_secret_key() -> &[u8] {
    match env::var("JWT_SECRET_KEY"){
        Ok(val) => return val,
        Err(e) => unimplemented!("{}", e)
    }
}

const SECRET: &[u8] = b"{get_secret_key()}";

pub fn validate_jwt(token: &str) -> Result<String, Error> {
    let token_data: TokenData<Claims> = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_data.claims.sub)
}
