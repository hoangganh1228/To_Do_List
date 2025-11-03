use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
  #[default]
  Pending,
  InProgress,
  Completed,
  Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum TaskPriority {
  #[default]
  Low, 
  Medium,
  High,
  Urgent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  
  pub user_id: ObjectId,
  
  pub title: String,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub due_date: Option<DateTime<Utc>>,
  
  #[serde(default)]
  pub status: TaskStatus,
  
  #[serde(default)]
  pub priority: TaskPriority,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created_by: Option<ObjectId>,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_by: Option<ObjectId>,
  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created_at: Option<DateTime<Utc>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<DateTime<Utc>>,
}


#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
  pub title: String,
  pub description: Option<String>,
  pub status: TaskStatus,
  pub priority: TaskPriority,
  pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
  pub title: Option<String>,
  pub description: Option<String>,
  pub status: Option<TaskStatus>,
  pub priority: Option<TaskPriority>,
  pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
  pub id: String,
  pub user_id: String,
  pub title: String,
  pub description: Option<String>,
  pub status: TaskStatus,
  pub priority: TaskPriority,
  pub due_date: Option<DateTime<Utc>>,
  pub created_by: Option<String>,
  pub updated_by: Option<String>,
  pub created_at: Option<DateTime<Utc>>,
  pub updated_at: Option<DateTime<Utc>>,
}