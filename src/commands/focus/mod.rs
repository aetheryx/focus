pub mod start;
use crate::context::{Context, Error};

#[poise::command(slash_command, subcommands("start::start"))]
pub async fn focus(_ctx: Context<'_>) -> Result<(), Error> {
  // no root command
  Ok(())
}
