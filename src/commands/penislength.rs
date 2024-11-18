use chrono::{Datelike, Local};
use murmur3;
use poise::serenity_prelude::{self as serenity, UserId};

use crate::context::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn penislength(
  ctx: Context<'_>,
  #[description = "The user who's penis length you want to see"] user: Option<serenity::User>,
) -> Result<(), Error> {
  let user = user.as_ref().unwrap_or_else(|| ctx.author());
  let penis_length = calculate_penis_length(user.id)
    .expect("failed to calculate penis length: penis size is too big");

  let title = format!("{}'s penis", user.name);
  let mut shaft = "=".repeat(penis_length);
  if penis_length > 0 {
    shaft.push('D');
  }
  let description = format!(
    "## 8{}\nIt's a whopping **{} centimeters**!",
    shaft,
    penis_length
  );

  let embed = serenity::CreateEmbed::new()
    .author(serenity::CreateEmbedAuthor::new(title).icon_url(user.face()))
    .description(description)
    .footer(serenity::CreateEmbedFooter::new("Penis size resets every week"));

  let builder = poise::CreateReply::default().embed(embed);

  ctx.send(builder).await?;

  Ok(())
}


const SEED: u32 = u32::from_be_bytes(*b"CUM.");
const MAX_PENIS_LENGTH: u32 = 25;

fn calculate_penis_length(id: UserId) -> Result<usize, Error> {
  /*
   * How this function works:
   * Discord snowflakes (user IDs) are 64 bit integers, with certain bit ranges dedicated to certain values
   * (see https://discord.com/developers/docs/reference#snowflakes-snowflake-id-format-structure-left-to-right)
   * Some of these values (especially towards the LSB) are very biased, which is why `id % n` is not a uniform hash.
   * The 42 most significant bits of the snowflake contain the timestamp (milliseconds with epoch), which is the
   * most unbiased variable. These 42 bits are used for the first 6 bytes of the input.
   * The 7th byte of the input is `current_year % 100` (e.g. 24 for 2024).
   * The 8th byte of the input is the current week of the year (1 >= x >= 53).
   *
   * This way, users are hashed according to three variables:
   * - The millisecond they made their account in (bytes [0, 5])
   * - The current week (byte 7)
   * - The current year of the century (byte 8)
   * 
   * These 8 bytes of input data are passed through murmurhash3 (https://wikipedia.org/wiki/MurmurHash) with a
   * constant seed, which is "CUM." in ASCII representation, interpreted as a big endian 32 bit integer.
   * The resulting hash is then modulo'd against the max penis size, returning a usize integer [0, 25).
   */

  let input = {
    let mut input = [0u8; 8];
    let now = Local::now();

    let user_id_bytes = u64::from(id).to_le_bytes();
    input[0..5].copy_from_slice(&user_id_bytes[0..5]);

    input[6] = (now.year() % 100).try_into()?;
    input[7] = now.iso_week().week().try_into()?;

    input
  };

  let hashed = murmur3::murmur3_32(&mut input.as_ref(), SEED)?;
  let length = hashed % MAX_PENIS_LENGTH;

  Ok(length.try_into()?)
}