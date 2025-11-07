use axum::{
  http::StatusCode, 
  extract::{State, Path, Query},
  response::{Json, IntoResponse},
};
use mongodb::bson::oid::ObjectId;
use futures_util::StreamExt;
use crate::{
  db::AppState,
  models::{User},
  dtos::{CreateUserRequest, UpdateUserRequest, UserResponse, LoginRequest, LoginResponse},  // Import DTOs từ dtos
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;

use crate::auth::jwt::Claims;
use crate::auth::jwt::create_token;
use bcrypt::verify;
use std::time::Instant;
use crate::auth::middleware::{AuthenticatedUser, RoleGuard};
use crate::utils::{ResultExt, AppError};

pub async fn create_user(
  State(app_state): State<AppState>,
  admin: AuthenticatedUser,
  Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
  admin.require_role(0).map_err(|e| AppError::forbidden(e.message))?;

  let admin_object_id = ObjectId::parse_str(&admin.user_id)
      .bad_request("Invalid user ID")?;

  let collection = app_state.db.collection::<User>("users");
  
  let existing_user_filter = mongodb::bson::doc! { 
    "email": &payload.email,
    "deleted": false
  };

  let existing_user = collection.find_one(existing_user_filter).await
      .internal_error("Failed to query database")?;
  
  if existing_user.is_some() {
    return Err(AppError::conflict("User already exists"));
  }

  // Get the "users" collection from MongoDB, mapping each document to the User struct
  let user = User {
    id: None,
    full_name: payload.full_name,
    email: payload.email,
    password: hash(payload.password, DEFAULT_COST)
        .internal_error("Failed to hash password")?,
    role: payload.role,
    created_by: Some(admin_object_id),
    updated_by: Some(admin_object_id),
    deleted:false,
    created_at: Some(Utc::now()),
    updated_at: Some(Utc::now()),
  };

  let result = collection.insert_one(&user).await
      .internal_error("Failed to insert user into database")?;
  
  let user_id = result.inserted_id.as_object_id()
      .ok_or_else(|| AppError::internal_error("Failed to get inserted user ID"))?;
  
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
) -> Result<Json<UserResponse>, AppError> {
  
  let collection = app_state.db.collection::<User>("users");
  let filter = mongodb::bson::doc! { 
    "_id": id,
    "deleted": false,
  };

  let user = collection.find_one(filter).await
      .internal_error("Failed to query database")?;
  if user.is_none() {
    return Err(AppError::not_found("User not found"));
  }
  let user = user.unwrap();
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
) -> Result<Json<Vec<UserResponse>>, AppError> {
  let collection = app_state.db.collection::<User>("users");

  let filter = mongodb::bson::doc! { "deleted": false };

  let mut cursor = collection.find(filter).await
      .internal_error("Failed to query database")?;
  let mut users = Vec::new();
  while let Some(user_result) = cursor.next().await
  {
    let user = user_result.internal_error("Failed to query database")?;
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

pub async fn update_user(
  State(app_state): State<AppState>,
  admin: AuthenticatedUser,
  Path(id): Path<ObjectId>,
  Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
  admin.require_role(0).map_err(|e| AppError::forbidden(e.message))?;

  let collection = app_state.db.collection::<User>("users");
  
  let admin_object_id = ObjectId::parse_str(&admin.user_id)
      .bad_request("Invalid user ID")?;
  
  let filter = mongodb::bson::doc! { 
    "_id": id,
    "deleted": false
  };
  let existing_user = collection.find_one(filter.clone())
      .await
      .internal_error("Failed to query database")?
      .ok_or_else(|| AppError::not_found("User not found"))?;
  let updated_user = User {
    id: Some(id),
    full_name: payload.full_name.unwrap_or(existing_user.full_name),
    email: payload.email.unwrap_or(existing_user.email),
    password: payload.password.unwrap_or(existing_user.password),
    role: payload.role.unwrap_or(existing_user.role),
    deleted: existing_user.deleted,
    created_by: Some(admin_object_id),
    updated_by: Some(admin_object_id),
    created_at: existing_user.created_at,
    updated_at: Some(Utc::now()),
  };
  collection.replace_one(filter, &updated_user)
      .await
      .internal_error("Failed to update user in database")?;
      
  let response = UserResponse {
    id: id.to_hex(),
    full_name: updated_user.full_name,
    email: updated_user.email,
    role: updated_user.role,
    created_by: updated_user.created_by.map(|id| id.to_hex()),
    updated_by: updated_user.updated_by.map(|id| id.to_hex()),
    created_at: updated_user.created_at,
    updated_at: updated_user.updated_at,
  };
  Ok(Json(response))
}

pub async fn login(
  State(app_state): State<AppState>,
  Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
  let collection = app_state.db.collection::<User>("users");

  let filter = mongodb::bson::doc! { 
    "email": payload.email, 
    "deleted": false 
  };

  let start = Instant::now();

  let user = collection.find_one(filter).await
      .internal_error("Failed to query database")?
      .ok_or_else(|| AppError::not_found("User not found"))?;
  
  verify(&payload.password, &user.password)
    .internal_error("Failed to verify password")?;
  
    if !verify(&payload.password, &user.password)
      .internal_error("Failed to verify password")? {
    }


  let user_id = user.id.unwrap().to_hex();
  let claims = Claims::new(
    user_id.clone(),
    user.email.clone(),
    user.role,
  );
  
  let token = create_token(claims)
      .internal_error("Failed to create token")?;
  let response = LoginResponse { token };
  println!("⏱️ Handler took {:?}", start.elapsed());
  Ok(Json(response))
}

