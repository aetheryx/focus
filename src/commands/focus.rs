use poise::serenity_prelude as serenity;
use chrono::{Local, Duration};

use crate::context::{Context, Error};
use crate::session;

#[poise::command(slash_command, prefix_command)]
pub async fn focus(
  ctx: Context<'_>,
  #[description = "The amount of hours you need to focus"] hours: Option<u32>,
  #[description = "The amount of minutes you need to focus"] minutes: Option<u32>,
) -> Result<(), Error> {
  let duration = Duration::hours(hours.unwrap_or(0).into()) +
    Duration::minutes(minutes.unwrap_or(0).into());

  if duration.is_zero() {
    let builder = poise::CreateReply::default()
      .content("err: must supply hours or minutes (or both)")
      .ephemeral(true);

    ctx.send(builder).await?;
    return Ok(());
  }

  let end = Local::now() + duration;

  let content = format!(
    "Are you sure you want to go into focus until <t:{:?}:F>?\nYou will not be able to leave focus mode voluntarily.",
    end.timestamp()
  );

  let components = serenity::CreateActionRow::Buttons(vec![
    serenity::CreateButton::new("confirm")
      .label("Yes, I'm sure")
      .style(serenity::ButtonStyle::Danger),
    serenity::CreateButton::new("cancel")
      .label("Cancel")
      .style(serenity::ButtonStyle::Secondary),
  ]);

  let options = poise::CreateReply::default()
    .content(content)
    .components(vec![components.clone()])
    .ephemeral(true);

  let reply = ctx.send(options).await?;
  let handle = reply.message().await?;

  let interaction = handle.await_component_interaction(ctx).await;
  let Some(interaction) = interaction else {
    return Ok(());
  };

  if !interaction.data.custom_id.eq("confirm") {
    let builder = serenity::CreateInteractionResponse::UpdateMessage(
      serenity::CreateInteractionResponseMessage::new()
        .content("Cancelled")
        .components(vec![])
    );

    interaction.create_response(ctx, builder).await?;
    return Ok(());
  }

  let builder = serenity::CreateInteractionResponse::UpdateMessage(
    serenity::CreateInteractionResponseMessage::new()
      .content("Removing access...")
      .components(vec![])
  );

  interaction.create_response(ctx, builder).await?;
  session::create_session(ctx, end).await?;
  
  Ok(())
}
