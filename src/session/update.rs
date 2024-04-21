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
    user_id: ActiveValue::NotSet,
    expires_at: ActiveValue::Set(expires_at),
    summarize: ActiveValue::NotSet,
  })
    .exec(&ctx.data().db)
    .await?;

  Ok(())
}
