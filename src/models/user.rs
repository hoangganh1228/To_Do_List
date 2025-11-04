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
