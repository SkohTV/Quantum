use crate::discord;
use poise::serenity_prelude as serenity;

pub enum LogRole {
    Error,
    Success,
    // Info,
}


pub async fn log_to_discord(
    ctx: &crate::discord::Context<'_>,
    log_message: String,
    role: LogRole, 
) -> () {

    let color = match role {
        LogRole::Error => serenity::model::Color::RED,
        LogRole::Success => serenity::model::Color::FOOYOO,
        // LogRole::Info => serenity::Colour::BLURPLE,
    };

    let embed = discord::default::embed()
        .timestamp(serenity::Timestamp::now())
        .description(log_message)
        .color(color);

    let msg = serenity::CreateMessage::default()
        .embed(embed);

    let _ = serenity::ChannelId::from(discord::ids::LOG_CHANNEL).send_message(ctx, msg).await;
}
