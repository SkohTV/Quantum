use crate::{Context, Error};
use poise::serenity_prelude as serenity;



/// Send an embed from JSON data into a channel
#[poise::command(
    slash_command,
    rename="embed",
)]
pub async fn cmd(
    ctx: Context<'_>,
    #[description = "Destination channel"] channel: serenity::GuildChannel,
    #[description = "JSON embed data"] data: String
) -> Result<(), Error> {

    let embed: serenity::Embed = serde_json::from_str(data.as_str())?;
    let msg = serenity::CreateMessage::default()
        .embed(embed.into());

    let _ = channel.send_message(ctx, msg).await?;

    Ok(())
}
