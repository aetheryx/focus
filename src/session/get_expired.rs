use sea_orm::*;
use chrono::Local;

use crate::db::entities::*;

pub async fn get_expired(db: &DatabaseConnection) -> anyhow::Result<Vec<focus_session::Model>> {
  let now = Local::now().naive_local();

  let sessions = focus_session::Entity::find()
    .filter(focus_session::Column::ExpiresAt.lte(now))
    .all(db)
    .await?;

  Ok(sessions)
}
