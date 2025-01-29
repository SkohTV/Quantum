use crate::discord::{Context, Error};
use poise::serenity_prelude as serenity;



/// Send an embed from JSON data into a channel
#[poise::command(
    slash_command,
    rename="embed",
    default_member_permissions="ADMINISTRATOR",
)]
pub async fn cmd(
    ctx: Context<'_>,
    #[description = "Destination channel"] channel: serenity::GuildChannel,
    #[description = "JSON embed data"] data: String
) -> Result<(), Error> {

    let text = format!("⌛ Loading...");
    let response = ctx.say(text).await?;

    let embed: serenity::Embed = serde_json::from_str(data.as_str())?;
    let msg = serenity::CreateMessage::default()
        .embed(embed.into());

    channel.send_message(ctx, msg).await?;

    let msg = poise::CreateReply::default()
        .content("Successfully sent embed")
        .attachment(serenity::CreateAttachment::bytes(
            data, "data.txt"
        ));

    response.edit(ctx, msg).await?;

    Ok(())
}
