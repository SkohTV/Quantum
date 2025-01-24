use poise::serenity_prelude as serenity;
use crate::consts;
use crate::discord::{commands, framework, Data};





pub async fn app() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let status = serenity::OnlineStatus::Online;
    let activity = serenity::ActivityData::playing(consts::VERSION);


    let options = poise::FrameworkOptions {

        commands: vec![
            commands::ping::cmd(),
            commands::embed::cmd(),
        ],

        post_command: |ctx| Box::pin(framework::post_command(ctx)),
        on_error: |err| Box::pin(framework::on_error(err)),

        ..Default::default()
    };


    let framework = poise::Framework::builder()
        .options(options)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
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
