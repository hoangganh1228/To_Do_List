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


pub async fn create_user(
  State(app_state): State<AppState>,
  admin: AuthenticatedUser,
  Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
  admin.require_role(0).map_err(|_| StatusCode::FORBIDDEN)?;

  let admin_object_id = ObjectId::parse_str(&admin.user_id)
      .map_err(|_| StatusCode::BAD_REQUEST)?;

  let collection = app_state.db.collection::<User>("users");
  
  let existing_user_filter = mongodb::bson::doc! { 
    "email": &payload.email,
    "deleted": false
  };

  let existing_user = collection.find_one(existing_user_filter).await
      .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
  
  if existing_user.is_some() {
    return Err(StatusCode::CONFLICT);
  }
  

  // Get the "users" collection from MongoDB, mapping each document to the User struct
  let user = User {
    id: None,
    full_name: payload.full_name,
    email: payload.email,
    password: hash(payload.password, DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    role: payload.role,
    created_by: Some(admin_object_id),
    updated_by: Some(admin_object_id),
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

pub async fn login(
  State(app_state): State<AppState>,
  Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
  let collection = app_state.db.collection::<User>("users");

  let filter = mongodb::bson::doc! { 
    "email": payload.email, 
    "deleted": false 
  };

  let start = Instant::now();

  let user = collection.find_one(filter).await
      .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
      .ok_or(StatusCode::NOT_FOUND)?;
  
  verify(&payload.password, &user.password)
    .map_err(|_| StatusCode::UNAUTHORIZED)?;
  
    if !verify(&payload.password, &user.password)
      .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
      return Err(StatusCode::UNAUTHORIZED);
    }


  let user_id = user.id.unwrap().to_hex();
  let claims = Claims::new(
    user_id.clone(),
    user.email.clone(),
    user.role,
  );
  
  let token = create_token(claims).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
  let response = LoginResponse { token };
  println!("⏱️ Handler took {:?}", start.elapsed());
  Ok(Json(response))
}

