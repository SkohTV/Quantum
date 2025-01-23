pub mod ids;
pub mod default;
pub mod logging;


pub const VERSION: &str = "5.0.0-β";


pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
