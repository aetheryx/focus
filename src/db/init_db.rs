use sea_orm::{Database, DatabaseConnection};

use crate::config::*;

pub async fn init_db() -> anyhow::Result<DatabaseConnection> {
  let database_url = DATABASE_URL.clone();
  let db = Database::connect(database_url).await?;
  Ok(db)
}

