use sea_orm::EntityTrait;

use crate::{
  db::entities::*,
  context::Context
};

pub async fn get_all(ctx: Context<'_>) -> anyhow::Result<Vec<focus_session::Model>> {
  let sessions = focus_session::Entity::find()
    .all(&ctx.data().db)
    .await?;

  return Ok(sessions)
}
