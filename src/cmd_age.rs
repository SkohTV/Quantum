use crate::{Context, Error};



#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {

    let start = std::time::Instant::now();

    let text = format!("⌛ Loading...");
    let response = ctx.say(text).await?;

    let end = std::time::Instant::now();

    let diff = end - start;
    let text =  format!("Discord Websocket ⇒ `{}.{}ms`", diff.as_secs(), diff.subsec_millis());
    let msg = poise::CreateReply::default();
    let msg = msg.content(text);

    response.edit(ctx, msg).await?;

    Ok(())
}
