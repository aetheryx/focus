use crate::context::{Context, Error};
use poise::serenity_prelude::{self as serenity, Mentionable};
use super::get_generator;

const SIZE: u32 = 256;

#[poise::command(slash_command, prefix_command)]
pub async fn petpet(
  ctx: Context<'_>,
  #[description = "The user who you want to pet"] user: Option<serenity::User>,
) -> Result<(), Error> {
  let user = user.as_ref().unwrap_or_else(|| ctx.author());
  let avatar_url = if let Some(avatar) = user.avatar {
    format!("https://cdn.discordapp.com/avatars/{}/{}.png?size={}", user.id, avatar, SIZE)
  } else {
    let builder = poise::CreateReply::default()
      .content(format!("User {} has no avatar set, aborting.", user.mention()));

    ctx.send(builder).await?;
    return Ok(())
  };

  ctx.defer().await?;

  let generator = get_generator()?;
  let avatar = generator.get_avatar(avatar_url).await?;
  let gif = generator.generate_gif(&avatar)?;

  let builder = poise::CreateReply::default()
    .attachment(serenity::CreateAttachment::bytes(gif, "petpet.gif"));

  ctx.send(builder).await?;

  Ok(())
}
