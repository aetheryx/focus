mod context;
mod commands;
mod db;
mod client;
mod session;
mod config;
mod event_handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let _ = dotenvy::dotenv();

  let framework = poise::Framework::builder()
    .setup(|ctx, _ready, framework| {
      Box::pin(async move {
        poise::builtins::register_globally(ctx, &framework.options().commands).await?;

        let db = db::init().await?;
        let ctx = context::Data { db };
        Ok(ctx)
      })
    })
    .options(poise::FrameworkOptions {
      commands: vec![commands::focus(), commands::penislength(), commands::petpet()],
      event_handler: |_ctx, event, _framework, data| {
        Box::pin(event_handler::handle(event, data))
      },
      ..Default::default()
    })
    .build();

  tokio::spawn(session::poll_thread());

  let mut client = client::client_builder()
    .framework(framework)
    .await?;

  client.start().await.unwrap();

  Ok(())
}
