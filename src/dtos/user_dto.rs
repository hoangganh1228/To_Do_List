use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
  pub full_name: String,
  pub email: String,
  pub password: String,
  pub role: i16,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
  pub full_name: Option<String>,
  pub email: Option<String>,
  pub password: Option<String>,
  pub role: Option<i16>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
  pub id: String,
  pub full_name: String,
  pub email: String,
  pub role: i16,
  pub created_by: Option<String>,
  pub updated_by: Option<String>,
  pub created_at: Option<DateTime<Utc>>,
  pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
  pub token: String,
}
