use std::str::FromStr;
use poise::serenity_prelude::{self as serenity, GuildId, CreateEmbed};
use poise::CreateReply;

use crate::{
  context::{Context, Error},
  session,
  config::*,
};

#[poise::command(slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
  let mut embed = CreateEmbed::default();
  let sessions = session::get_all(ctx).await?;

  for session in sessions {
    let timestamp = session.expires_at.timestamp();
    let user_id = serenity::UserId::new(session.user_id as u64);
    let guild_id = GuildId::from_str(&GUILD_ID)?;
    let user = ctx.http()
      .get_member(guild_id, user_id)
      .await?;

    embed = embed.field(
      format!("@{}", user.display_name()),
      if session.hide {
        format!("- Ends at: ? (?)")
      } else {
        format!("- Ends at: <t:{}:F> (<t:{}:R>)", timestamp, timestamp)
      },
      false
    );
  }

  // Whether the author is in a focus session
  let ephemeral = session::get_session(ctx, ctx.author().id).await?.is_some();

  let builder = CreateReply::default()
    .embed(embed)
    .ephemeral(ephemeral);

  ctx.send(builder).await?;
  Ok(())
}
