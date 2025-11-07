use axum::{
  http::StatusCode,
  response::{IntoResponse, Response, Json},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
  pub message: String,  
}

pub struct AppError {
  pub status: StatusCode,
  pub message: String, 
}

impl AppError {
  pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
    Self {
      status,
      message: message.into(),
    }
  }


  pub fn bad_request(message: impl Into<String>) -> Self {
    Self::new(StatusCode::BAD_REQUEST, message)
  }

  pub fn unauthorized(message: impl Into<String>) -> Self {
    Self::new(StatusCode::UNAUTHORIZED, message)
  }

  pub fn forbidden(message: impl Into<String>) -> Self {
    Self::new(StatusCode::FORBIDDEN, message)
  }

  pub fn not_found(message: impl Into<String>) -> Self {
    Self::new(StatusCode::NOT_FOUND, message)
  }

  pub fn conflict(message: impl Into<String>) -> Self {
    Self::new(StatusCode::CONFLICT, message)
  }

  pub fn internal_error(message: impl Into<String>) -> Self {
    Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let body = Json(ErrorResponse {
      message: self.message,
    });
    (self.status, body).into_response()
  }
}