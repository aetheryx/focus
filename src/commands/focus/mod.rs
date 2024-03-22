pub mod start;
pub mod list;

use crate::context::{Context, Error};

#[poise::command(slash_command, subcommands("start::start", "list::list"))]
pub async fn focus(_ctx: Context<'_>) -> Result<(), Error> {
  // no root command
  Ok(())
}
