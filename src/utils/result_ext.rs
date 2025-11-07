use super::errors::AppError;   // use Struct
use axum::http::StatusCode;

pub trait ResultExt<T, E> {
  fn app_error(self, status: StatusCode, error: impl Into<String>) -> Result<T, AppError>;
  fn internal_error(self, msg: impl Into<String>) -> Result<T, AppError>;
  fn bad_request(self, msg: impl Into<String>) -> Result<T, AppError>;
  fn conflict(self, msg: impl Into<String>) -> Result<T, AppError>;
  fn not_found(self, msg: impl Into<String>) -> Result<T, AppError>;
  fn forbidden(self, msg: impl Into<String>) -> Result<T, AppError>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
  fn app_error(self, status: StatusCode, error: impl Into<String>) -> Result<T, AppError> {
    self.map_err(|_| AppError::new(status, error))
  }
  fn internal_error(self, msg: impl Into<String>) -> Result<T, AppError> {
    self.map_err(|_| AppError::internal_error(msg))
  }
  fn bad_request(self, msg: impl Into<String>) -> Result<T, AppError> {
    self.map_err(|_| AppError::bad_request(msg))
  }
  fn conflict(self, msg: impl Into<String>) -> Result<T, AppError> {
    self.map_err(|_| AppError::conflict(msg))
  }
  fn not_found(self, msg: impl Into<String>) -> Result<T, AppError> {
    self.map_err(|_| AppError::not_found(msg))
  }
  fn forbidden(self, msg: impl Into<String>) -> Result<T, AppError> {
    self.map_err(|_| AppError::forbidden(msg))
  }
}