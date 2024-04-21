pub mod start;
pub mod list;
pub mod extend;

use crate::context::{Context, Error};

#[poise::command(slash_command, subcommands("start::start", "list::list", "extend::extend"))]
pub async fn focus(_ctx: Context<'_>) -> Result<(), Error> {
  // no root command
  Ok(())
}
