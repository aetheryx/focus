use std::{str::FromStr, time::Duration};
use poise::serenity_prelude::{Client, GuildId, RoleId, UserId};
use sea_orm::{DatabaseConnection, EntityTrait};
use tokio::time;

use crate::{
  client::client_builder,
  db::entities::*,
  session::*,
  config::*,
  db,
};

pub async fn poll_thread() -> anyhow::Result<()> {
  let client = client_builder().await?;
  let db = db::init().await?;

  loop {
    let res = tick(&client, &db).await;
    if let Result::Err(err) = res {
      println!("failed to handle tick: {err:?}");
    };

    time::sleep(Duration::from_secs(1)).await;
  }
}

async fn tick(client: &Client, db: &DatabaseConnection) -> anyhow::Result<()> {
  let sessions = get_expired(db).await?;

  for session in sessions {
    handle_inactive_session(client, db, session).await?;
  }

  Ok(())
}

async fn handle_inactive_session(
  client: &Client,
  db: &DatabaseConnection,
  session: focus_session::Model,
) -> anyhow::Result<()> {

  println!("handling {session:?}");
  let guild_id = GuildId::from_str(&GUILD_ID)?;
  let user_id = UserId::new(session.user_id.try_into().unwrap());
  let role_id = RoleId::from_str(&FOCUS_ROLE_ID)?;

  client.http
    .remove_member_role(guild_id, user_id, role_id, Some("Entering focus mode"))
    .await?;

  focus_session::Entity::delete_by_id(session.id)
    .exec(db)
    .await?;

  Ok(())
}
