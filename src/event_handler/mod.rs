use poise::serenity_prelude as serenity;

use crate::context::{Data, Error};

pub async fn handle(
  event: &serenity::FullEvent,
  data: &Data,
) -> Result<(), Error> {
  match event {
    _ => {}
  }

  Ok(())
}

