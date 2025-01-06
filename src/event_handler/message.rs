use poise::serenity_prelude as serenity;
use std::str::FromStr;
use anyhow::*;

use sea_orm::{EntityTrait, ActiveValue};
use serenity::all::ChannelId;
use serenity::model::channel::Message;

use crate::config::*;
use crate::context::Data;
use crate::db::entities::*;

pub async fn message(data: &Data, msg: &Message) -> Result<()> {
  let cat_channel_id = ChannelId::from_str(&CAT_CHANNEL_ID)?;
  if msg.channel_id != cat_channel_id {
    return Ok(());
  }

  if msg.attachments.is_empty() {
    return Ok(());
  }

  let models = msg.attachments.iter()
    .map(|attachment| cat_image::ActiveModel {
      message_id: ActiveValue::Set(msg.id.into()),
      attachment_id: ActiveValue::Set(attachment.id.into()),
      ..Default::default()
    });

  println!("inserting cat pics: {:?}", models);
  cat_image::Entity::insert_many(models).exec(&data.db).await?;

  Ok(())
}