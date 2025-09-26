use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};

use crate::error::AuthError;

pub type TelegramUserId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: TelegramUserId,
    pub exp: usize,
}

impl Claims {
    fn get_secret() -> String {
        dotenv::var("JWT_SECRET").unwrap_or_else(|_| "testing".to_string())
    }

    pub fn token(&self) -> Option<String> {
        let secret = Self::get_secret();
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .ok()
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() as usize >= self.exp
    }
}

impl From<TelegramUserId> for Claims {
    fn from(user_id: TelegramUserId) -> Self {
        let expiration = Utc::now()
            .checked_add_signed(Duration::weeks(52))
            .expect("valid timestamp")
            .timestamp() as usize;
        Claims {
            sub: user_id,
            exp: expiration,
        }
    }
}

pub struct JsonWebToken(String);

impl JsonWebToken {
    pub fn new(token: String) -> Self {
        JsonWebToken(token)
    }

    /// Decodes a `token`. Fails if the `token` is invalid, empty or expired.
    pub fn decode(&self) -> Result<TokenData<Claims>, AuthError> {
        let secret = Claims::get_secret();

        let validator = Validation::new(Algorithm::HS256);

        let token = self.0.clone();
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        decode::<Claims>(token.as_str(), &decoding_key, &validator)
            .map_err(|_| AuthError::TokenNotGenerated)
            .and_then(|f| {
                if f.claims.is_expired() {
                    Err(AuthError::ExpiredToken)
                } else {
                    Ok(f)
                }
            })
    }
}
