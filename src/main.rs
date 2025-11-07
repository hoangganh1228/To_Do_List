mod db;
mod models;
mod handlers;
mod routes;
mod dtos;
mod auth;
mod utils;

use axum::Router;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;
use db::{get_database, AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database = get_database()
        .await
        .expect("Failed to connect to database");

    let app_state = AppState::new(database);

    let app = routes::create_router(app_state)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}