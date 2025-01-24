use crate::discord;



pub async fn on_error(error: poise::FrameworkError<'_, discord::Data, discord::Error>) {

    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            let msg = format!(
                "{} in <#{}>\n➜ `/{}`\n\n```{}```",
                ctx.author(),
                ctx.channel_id(),
                ctx.command().qualified_name,
                error
            );

            discord::logging::log_to_discord(&ctx, msg, discord::logging::LogRole::Error).await;
            let _ = ctx.send(poise::CreateReply::default()
                .content(format!("An error happened\n```{}```", error))
                .ephemeral(true)
            ).await;
        },
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}



pub async fn post_command(ctx: discord::Context<'_>) {
    let msg = format!(
        "{} in <#{}>\n➜ `/{}`\nSuccess",
        ctx.author(),
        ctx.channel_id(),
        ctx.command().qualified_name,
    );

    discord::logging::log_to_discord(&ctx, msg, discord::logging::LogRole::Success).await;
}
