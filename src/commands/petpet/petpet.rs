use crate::context::{Context, Error};
use poise::serenity_prelude::{self as serenity};
use super::get_generator;

#[poise::command(slash_command, prefix_command)]
pub async fn petpet(
  ctx: Context<'_>,
  #[description = "The user who you want to pet"] user: Option<serenity::User>,
) -> Result<(), Error> {
  ctx.defer().await?;

  let generator = get_generator()?;

  let user = user.as_ref().unwrap_or_else(|| ctx.author());
  let avatar_url = user.static_face().replace("webp", "png");
  let avatar = generator.get_avatar(avatar_url).await?;
  let gif = generator.generate_gif(&avatar)?;

  let builder = poise::CreateReply::default()
    .attachment(serenity::CreateAttachment::bytes(gif, "petpet.gif"));

  ctx.send(builder).await?;

  Ok(())
}
