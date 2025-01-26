pub mod ids;
pub mod default;
pub mod logging;
pub mod framework;
pub mod app;
pub mod commands;





pub struct Data {}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
