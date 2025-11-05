use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
  pub user_id: String,
  pub email: String,
  pub role: i16,
  pub exp: usize,
}

impl Claims {
  pub fn new(user_id: String, email: String, role: i16) -> Self {
    let exp = (Utc::now() + Duration::hours(1)).timestamp() as usize;
    Self { user_id, email, role, exp }
  }
}

pub fn create_token(claims: Claims) -> Result<String, jsonwebtoken::errors::Error> {
  let secret = env::var("JWT_SECRET")
      .expect("JWT_SECRET must be set");
  
  encode(
    &Header::default(), 
    &claims, 
    &EncodingKey::from_secret(secret.as_bytes())
  )
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
  let secret = env::var("JWT_SECRET")
      .expect("JWT_SECRET must be set");
  
  let validation = Validation::default();
  let token_data = decode::<Claims>(
    token, 
    &DecodingKey::from_secret(secret.as_bytes()), 
    &validation
  )?;

  Ok(token_data.claims)
}