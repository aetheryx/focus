mod context;
mod commands;
mod db;
mod client;
mod session;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenvy::dotenv()?;

  let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
      commands: vec![commands::focus::focus()],
      ..Default::default()
    })
    .setup(|ctx, _ready, framework| {
      Box::pin(async move {
        poise::builtins::register_globally(ctx, &framework.options().commands).await?;

        let db = db::init().await?;
        let ctx = context::Data { db };
        Ok(ctx)
      })
    })
    .build();

  tokio::spawn(session::poll_thread());

  let mut client = client::client_builder()
    .framework(framework)
    .await?;

  client.start().await.unwrap();

  Ok(())
}
