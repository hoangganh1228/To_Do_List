use axum::{
  http::StatusCode, 
  extract::{State, Path, Query},
  response::{Json, IntoResponse},
};
use futures_util::StreamExt;
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use crate::{
  db::AppState,
  models::{Task, TaskStatus, TaskPriority},
  dtos::{CreateTaskRequest, UpdateTaskRequest, TaskResponse},
};
use chrono::{DateTime, Utc};
use crate::utils::{ResultExt, AppError};


#[derive(Deserialize)]
pub struct TaskQuery {
  pub user_id: Option<ObjectId>,
  pub status: Option<TaskStatus>,
  pub priority: Option<TaskPriority>,
}

pub async fn create_task(
  State(app_state): State<AppState>,
  Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<TaskResponse>, AppError> {
  let collection = app_state.db.collection::<Task>("tasks");

  let user_object_id = ObjectId::parse_str(&payload.user_id)
      .bad_request("Invalid user ID")?;

  let task = Task {
    id: None,
    user_id: user_object_id,
    title: payload.title,
    description: payload.description,
    due_date: payload.due_date,
    status: payload.status,
    deleted: false,
    priority: payload.priority,
    created_by: None,
    updated_by: None,
    created_at: Some(Utc::now()),
    updated_at: Some(Utc::now()),
  };

  let result = collection.insert_one(&task).await
      .internal_error("Failed to insert task into database")?;
  
  let task_id = result.inserted_id.as_object_id()
      .ok_or_else(|| AppError::internal_error("Failed to get inserted task ID"))?;
  
  let response = TaskResponse {
    id: task_id.to_hex(),
    user_id: user_object_id.to_hex(),  
    title: task.title, 
    description: task.description,
    status: task.status, 
    priority: task.priority, 
    due_date: task.due_date,
    created_by: task.created_by.map(|id| id.to_hex()),      
    updated_by: task.updated_by.map(|id| id.to_hex()),  
    created_at: task.created_at,  
    updated_at: task.updated_at,
  };

  Ok(Json(response))
}

pub async fn get_task(
  State(app_state): State<AppState>,
  Path(id): Path<ObjectId>,
) -> Result<Json<TaskResponse>, AppError> {
  let collection = app_state.db.collection::<Task>("tasks");
  let filter = mongodb::bson::doc! { 
    "_id": id,
    "deleted": false
  };

  let task = collection.find_one(filter)
      .await
      .internal_error("Failed to query database")?;

  if task.is_none() {
    return Err(AppError::not_found("Task not found"));
  }
  let task = task.unwrap();
  let response = TaskResponse {
    id: task.id.unwrap().to_hex(),
    user_id: task.user_id.to_hex(),
    title: task.title,
    description: task.description,
    status: task.status,
    priority: task.priority,
    due_date: task.due_date,
    created_by: task.created_by.map(|id| id.to_hex()),
    updated_by: task.updated_by.map(|id| id.to_hex()),
    created_at: task.created_at,
    updated_at: task.updated_at,
  };

  Ok(Json(response))
}

pub async fn list_tasks(
  State(app_state): State<AppState>
) -> Result<Json<Vec<TaskResponse>>, StatusCode> {
  let collection = app_state.db.collection::<Task>("tasks");

  let filter = mongodb::bson::doc! { "deleted": false };
  
  let mut cursor = collection.find(filter).await
      .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
  let mut tasks = Vec::new();
  
  while let Some(task_result) = cursor.next().await
  {
    let task = task_result.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    tasks.push(TaskResponse {
      id: task.id.unwrap().to_hex(),
      user_id: task.user_id.to_hex(),
      title: task.title,
      description: task.description,
      status: task.status,
      priority: task.priority,
      due_date: task.due_date,
      created_by: task.created_by.map(|id| id.to_hex()),
      updated_by: task.updated_by.map(|id| id.to_hex()),
      created_at: task.created_at,
      updated_at: task.updated_at,
    });
  }

  Ok(Json(tasks))
}

pub async fn update_task(
  State(app_state): State<AppState>,
  Path(id): Path<String>,  
  Json(payload): Json<UpdateTaskRequest>,
) -> Result<Json<TaskResponse>, AppError> {
  let collection = app_state.db.collection::<Task>("tasks");

  let task_id = ObjectId::parse_str(&id)  
      .bad_request("Invalid task ID")?;

  let filter = mongodb::bson::doc! { 
    "_id": task_id,  
    "deleted": false 
  };
  
  let existing_task = collection
      .find_one(filter.clone())  
      .await
      .internal_error("Failed to query database")?
      .ok_or_else(|| AppError::not_found("Task not found"))?;

  let updated_task = Task {
    id: Some(task_id), 
    user_id: existing_task.user_id,
    title: payload.title.unwrap_or(existing_task.title),
    description: payload.description.or(existing_task.description),
    status: payload.status.unwrap_or(existing_task.status),
    deleted: existing_task.deleted,
    priority: payload.priority.unwrap_or(existing_task.priority),
    created_by: existing_task.created_by,
    updated_by: existing_task.updated_by,
    created_at: existing_task.created_at,
    updated_at: Some(Utc::now()),
    due_date: payload.due_date.or(existing_task.due_date),
  };

  collection
    .replace_one(filter, &updated_task)  
    .await
    .internal_error("Failed to update task in database")?;
  
  let response = TaskResponse {
    id: task_id.to_hex(),  
    user_id: updated_task.user_id.to_hex(),
    title: updated_task.title,
    description: updated_task.description,
    status: updated_task.status,
    priority: updated_task.priority,
    due_date: updated_task.due_date,
    created_by: updated_task.created_by.map(|id| id.to_hex()),
    updated_by: updated_task.updated_by.map(|id| id.to_hex()),
    created_at: updated_task.created_at,
    updated_at: updated_task.updated_at,
  };

  Ok(Json(response))
}

pub async fn delete_task(
  State(app_state): State<AppState>,
  Path(id): Path<ObjectId>,
) -> Result<StatusCode, AppError> {
  let collection = app_state.db.collection::<Task>("tasks");
  let filter = mongodb::bson::doc! { 
    "_id": id,
    "deleted": false
  };

  let result = collection.update_one(filter, mongodb::bson::doc! { "$set": { "deleted": true } })
    .await
    .internal_error("Failed to delete task in database")?;

  Ok(StatusCode::NO_CONTENT)
}

