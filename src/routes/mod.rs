use axum::{
  Router,
  routing::{get, post},
};

use crate:: {
  db::AppState,
  handlers,
};

pub fn create_router(app_state: AppState) -> Router
{
  Router::new()
    .route("/api/users", post(handlers::create_user))
    .route("/api/users/:id", get(handlers::get_user))
    .route("/api/users", get(handlers::list_users))
    .with_state(app_state)
}