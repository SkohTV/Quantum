use crate::utils::{Context, Error};



/// Display the latency of the bot
#[poise::command(
    slash_command,
    rename="ping",
)]
pub async fn cmd(ctx: Context<'_>) -> Result<(), Error> {

    let text = format!("⌛ Loading...");

    let start = std::time::Instant::now();
    let response = ctx.say(text).await?;
    let elapsed = start.elapsed();

    let text =  format!("Discord Websocket ⇒ `{}ms`", elapsed.as_millis());
    let msg = poise::CreateReply::default()
        .content(text);

    response.edit(ctx, msg).await?;

    Ok(())
}
