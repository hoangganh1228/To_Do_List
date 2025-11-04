use axum::{
  Router,
  routing::{get, post, delete, put},
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

    .route("/api/tasks", post(handlers::create_task))
    .route("/api/tasks/:id", get(handlers::get_task))
    .route("/api/tasks", get(handlers::list_tasks))
    .route("/api/tasks/:id", put(handlers::update_task))
    .route("/api/tasks/:id" , delete(handlers::delete_task))
    .with_state(app_state)
}
