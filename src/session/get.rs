use poise::serenity_prelude::UserId;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
  db::entities::*,
  context::Context
};

pub async fn get_session(ctx: Context<'_>, user_id: UserId) -> anyhow::Result<Option<focus_session::Model>> {
  let user_id = i64::from(user_id);

  let session = focus_session::Entity::find()
    .filter(focus_session::Column::UserId.eq(user_id))
    .one(&ctx.data().db)
    .await?;

  return Ok(session);
}
