use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub full_name: String,
  pub email: String,
  pub password: String,
  pub role:i16,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created_by: Option<ObjectId>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_by: Option<ObjectId>,
  pub deleted: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created_at: Option<DateTime<Utc>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTime<Utc>>,
}

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
 