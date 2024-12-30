use std::str::FromStr;
use chrono::{DateTime, Local};
use poise::serenity_prelude::{GuildId, RoleId};
use sea_orm::{EntityTrait, ActiveValue};

use crate::{
  db::entities::*,
  config::*,
  context::Context
};

pub async fn create_session(
  ctx: Context<'_>,
  end: DateTime<Local>,
) -> anyhow::Result<()> {
  focus_session::Entity::insert(focus_session::ActiveModel {
    id: ActiveValue::NotSet,
    user_id: ActiveValue::Set(ctx.author().id.into()),
    expires_at: ActiveValue::Set(end.naive_local()),
    summarize: ActiveValue::Set(false),
    hide: ActiveValue::NotSet,
  })
    .exec(&ctx.data().db)
    .await?;

  let user_id = ctx.author().id;
  let guild_id = GuildId::from_str(&GUILD_ID)?;
  let role_id = RoleId::from_str(&FOCUS_ROLE_ID)?;

  ctx.http()
    .add_member_role(guild_id, user_id, role_id, Some("Entering focus mode"))
    .await?;

  Ok(())
}
