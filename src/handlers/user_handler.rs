use axum::{
  http::StatusCode, 
  extract::{State, Path, Query},
  response::Json,
};
use mongodb::bson::oid::ObjectId;
use futures_util::StreamExt;
use crate::{
  db::AppState,
  models::{User},
  dtos::{CreateUserRequest, UpdateUserRequest, UserResponse},  // Import DTOs từ dtos
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;

pub async fn create_user(
  State(app_state): State<AppState>,
  Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {

  let collection = app_state.db.collection::<User>("users");   // Get the "users" collection from MongoDB, mapping each document to the User struct
  let user = User {
    id: None,
    full_name: payload.full_name,
    email: payload.email,
    password: hash(payload.password, DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    role: payload.role,
    created_by: None,
    updated_by: None,
    deleted:false,
    created_at: Some(Utc::now()),
    updated_at: Some(Utc::now()),
  };

  let result = collection.insert_one(&user).await
      .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
  
  let user_id = result.inserted_id.as_object_id()
      .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
  
  let response = UserResponse {
    id: user_id.to_hex(),
    full_name: user.full_name,
    email: user.email,
    role: user.role,
    created_at: user.created_at,
    updated_at: user.updated_at,
    created_by: user.created_by.map(|id| id.to_hex()),
    updated_by: user.updated_by.map(|id| id.to_hex()),
  };
  
  Ok(Json(response))
}

pub async fn get_user(
  State(app_state): State<AppState>,
  Path(id): Path<ObjectId>,
) -> Result<Json<UserResponse>, StatusCode> {
  
  let collection = app_state.db.collection::<User>("users");
  let filter = mongodb::bson::doc! { 
    "_id": id,
    "deleted": false,
  };

  let user = collection.find_one(filter).await
      .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
      .ok_or(StatusCode::NOT_FOUND)?;

  let response = UserResponse {
    id: user.id.unwrap().to_hex(),
    full_name: user.full_name,
    email: user.email,
    role: user.role,
    created_at: user.created_at,
    updated_at: user.updated_at,
    created_by: user.created_by.map(|id| id.to_hex()),
    updated_by: user.updated_by.map(|id| id.to_hex()),
  };

  Ok(Json(response))
}

pub async fn list_users(
  State(app_state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
  let collection = app_state.db.collection::<User>("users");

  let filter = mongodb::bson::doc! { "deleted": false };

  let mut cursor = collection.find(filter).await
      .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  let mut users = Vec::new();
  while let Some(user_result) = cursor.next().await
  {
    let user = user_result.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    users.push(UserResponse {
      id: user.id.unwrap().to_hex(),
      full_name: user.full_name,
      email: user.email,
      role: user.role,
      created_by: user.created_by.map(|id| id.to_hex()),
      updated_by: user.updated_by.map(|id| id.to_hex()),
      created_at: user.created_at,
      updated_at: user.updated_at,
    });
  }

  Ok(Json(users))
}

