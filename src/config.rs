use lazy_static::lazy_static;
use std::env;

lazy_static! {
  pub static ref DISCORD_TOKEN: String =
    env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");

  pub static ref FOCUS_ROLE_ID: String =
    env::var("FOCUS_ROLE_ID").expect("FOCUS_ROLE_ID must be set");

  pub static ref GUILD_ID: String =
    env::var("GUILD_ID").expect("GUILD_ID must be set");

  pub static ref DATABASE_URL: String =
    env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  pub static ref CAT_CHANNEL_ID: String =
    env::var("CAT_CHANNEL_ID").expect("CAT_CHANNEL_ID must be set");
}
