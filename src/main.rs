mod commands;
mod utils;

use dotenv::dotenv;
use poise::serenity_prelude as serenity;



async fn on_error(error: poise::FrameworkError<'_, utils::Data, utils::Error>) {

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

            utils::logging::log_to_discord(&ctx, msg, utils::logging::LogRole::Error).await;
        },
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}



#[tokio::main]
async fn main() {


    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let status = serenity::OnlineStatus::Online;
    let activity = serenity::ActivityData::playing(utils::VERSION);


    let options = poise::FrameworkOptions {

        commands: vec![
            commands::ping::cmd(),
            commands::embed::cmd(),
        ],

        post_command: |ctx| {
            Box::pin(async move {

                let msg = format!(
                    "{} in <#{}>\n➜ `/{}`\nSuccess",
                    ctx.author(),
                    ctx.channel_id(),
                    ctx.command().qualified_name,
                );

                utils::logging::log_to_discord(&ctx, msg, utils::logging::LogRole::Success).await;
            })
        },

        on_error: |err| Box::pin(on_error(err)),



        ..Default::default()
    };


    let framework = poise::Framework::builder()
        .options(options)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(utils::Data {})
            })
        })
        .build();


    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .status(status)
        .activity(activity)
        .await;

    client.unwrap().start().await.unwrap();
}
