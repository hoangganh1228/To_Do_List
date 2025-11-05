  use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::{request::Parts, StatusCode, HeaderMap},
    response::{Response, IntoResponse},
    Json,
    middleware::Next,
  };

  use serde::{Deserialize, Serialize};
  use crate::auth::jwt::{Claims, verify_token};

  // Represents an authentication error that can be returned to the client
  #[derive(Debug, Deserialize, Serialize)]
  pub struct AuthError {
    pub message: String,
  }

  /// Implement `IntoResponse` for `AuthError`
  /// This allows `AuthError` to be automatically converted into an HTTP response
  impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
      let body = Json(serde_json::json!({
        "error": self.message,
      }));
      (StatusCode::UNAUTHORIZED, body).into_response()
    }
  }

  // Struct that represents an authenticated user
  // If the request has a valid JWT, an instance of this struct will be created
  #[derive(Clone)]
  pub struct AuthenticatedUser {
    pub user_id: String,
    pub email: String,
    pub role: i16,
  }

  // Implement `FromRequestParts` for `AuthenticatedUser`
  // This makes `AuthenticatedUser` an **extractor**, meaning Axum can automatically
  // get it from the incoming request (like `req.user` in Express)
  #[async_trait]
  impl<S> FromRequestParts<S> for AuthenticatedUser         // Generic implementation for any state type S
  where
    S: Send + Sync,   // State must be thread-safe
  {
    type Rejection = AuthError;       // Error type returned if extraction fails

    async fn from_request_parts(
      parts: &mut Parts, 
      state: &S
    ) -> Result<Self, Self::Rejection> {
      let headers = &parts.headers;   // Contains request metadata (headers, URI, etc.)
      let auth_header = headers       
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AuthError { 
          message: "Missing Authorization header".to_string() 
        })?;
      
      if !auth_header.starts_with("Bearer ") {
        return Err(AuthError { 
          message: "Invalid Authorization header format".to_string(),
        });
      }
    
      let token = auth_header.split_at(7).1.to_string();
      let claims = verify_token(&token).map_err(|_| AuthError { 
        message: "Invalid token".to_string(),
      })?;
    
      Ok(AuthenticatedUser { 
        user_id: claims.user_id,
        email: claims.email,
        role: claims.role,
      })
    }
  }

  pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // Lấy Authorization header
    let auth_header = request.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AuthError {
            message: "Missing Authorization header".to_string(),
        })?;

    // Kiểm tra format "Bearer <token>"
    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError {
            message: "Invalid Authorization header format".to_string(),
        });
    }

    let token = auth_header.trim_start_matches("Bearer ");

    // Verify token
    let claims = verify_token(token).map_err(|_| AuthError {
        message: "Invalid or expired token".to_string(),
    })?;

    // Inject user info vào request extensions
    request.extensions_mut().insert(AuthenticatedUser {
        user_id: claims.user_id,
        email: claims.email,
        role: claims.role,
    });

    Ok(next.run(request).await)
}

