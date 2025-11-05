use axum::{
  Router,
  routing::{get, post, delete, put},
  middleware,
};

use crate:: {
  db::AppState,
  handlers,
  auth::middleware::auth_middleware,
};

pub fn create_router(app_state: AppState) -> Router {
  // Public routes (không cần authentication)
  let public_routes = Router::new()
    .route("/api/auth/login", post(handlers::login))
    .route("/api/users", post(handlers::create_user)); // Đăng ký

  // Protected routes (cần authentication)
  let protected_routes = Router::new()
    .route("/api/users/:id", get(handlers::get_user))
    .route("/api/users", get(handlers::list_users))
    .route("/api/tasks", post(handlers::create_task))
    .route("/api/tasks/:id", get(handlers::get_task))
    .route("/api/tasks", get(handlers::list_tasks))
    .route("/api/tasks/:id", put(handlers::update_task))
    .route("/api/tasks/:id", delete(handlers::delete_task))
    .layer(middleware::from_fn(auth_middleware)); 

  Router::new()
    .merge(public_routes)
    .merge(protected_routes)
    .with_state(app_state)
}
