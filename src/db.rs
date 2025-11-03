use mongodb::{Client, Database};
use std::env;
use dotenvy::dotenv;

pub async fn get_database() -> Result<Database, mongodb::error::Error> {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");

  let database_name = env::var("DATABASE_NAME")
    .expect("DATABASE_NAME must be set");

  let client = Client::with_uri_str(&database_url).await?;
  let db = client.database(&database_name);
  
  Ok(db)
}

#[derive(Clone)]
pub struct AppState {
  pub db: Database,
}

impl AppState {
  pub fn new(db: Database) -> Self {
      Self { db }
  }
}