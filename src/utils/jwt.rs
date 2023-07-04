use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};

use crate::{Result, Error};

#[derive(Clone)]
pub struct TokenSecret(pub &'static str);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    sub: i64,
}

impl Claims {
    pub fn new(user_id: i64) -> Self {
        Self {
            exp: (Utc::now() + Duration::hours(8)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
            iss: "LBK".to_owned(),
            sub: user_id,
        }
    }

    pub fn get_user_id(&self) -> i64 {
        self.sub
    }
}



/// Parse a token
/// Returns Claim
pub fn parse_token(token: String, secret: &str) -> Result<Claims> {
    // Parse the token with jsonwebtoken
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_error| Error::AuthFailTokenWrongFormat)
    .map(|token_data| token_data.claims)
}
