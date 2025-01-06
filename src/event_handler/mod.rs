use poise::serenity_prelude as serenity;

use crate::context::{Data, Error};
mod message;

pub async fn handle(
  event: &serenity::FullEvent,
  data: &Data,
) -> Result<(), Error> {
  match event {
    serenity::FullEvent::Message { new_message } =>
      message::message(data, new_message).await?,

    _ => {}
  }

  Ok(())
}

