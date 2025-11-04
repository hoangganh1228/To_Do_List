use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use crate::models::{TaskStatus, TaskPriority};

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
  pub user_id: String,
  pub title: String,
  pub description: Option<String>,
  #[serde(default)] 
  pub status: TaskStatus,
  #[serde(default)]
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