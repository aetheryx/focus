use sea_orm::DatabaseConnection;

pub struct Data {
  pub db: DatabaseConnection
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
