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
    .route("/users", post(handlers::create_user))
    .route("/users/:id", get(handlers::get_user))
    .with_state(app_state)
}