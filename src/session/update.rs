use chrono::NaiveDateTime;
use sea_orm::{ActiveValue, EntityTrait};

use crate::{context::Context, db::entities::*};

pub async fn update_session(
  ctx: Context<'_>,
  id: i64,
  expires_at: NaiveDateTime,
) -> anyhow::Result<()> {
  focus_session::Entity::update(focus_session::ActiveModel {
    id: ActiveValue::Set(id),
    expires_at: ActiveValue::Set(expires_at),
    ..Default::default()
  })
    .exec(&ctx.data().db)
    .await?;

  Ok(())
}
