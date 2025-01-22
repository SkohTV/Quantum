mod commands;
mod utils;
mod db;

use dotenv::dotenv;
use poise::serenity_prelude as serenity;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;



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

        pre_command: |ctx| {
            Box::pin(async move {

                let embed = utils::default::embed()
                    .timestamp(serenity::Timestamp::now())
                    .description(format!(
                        "{} in <#{}>\n➜ `{}`",
                        ctx.author(),
                        ctx.channel_id(),
                        ctx.command().qualified_name,
                    ));

                let msg = serenity::CreateMessage::default()
                    .embed(embed);

                let _ = serenity::ChannelId::from(utils::ids::LOG_CHANNEL).send_message(ctx, msg).await;
            })
        },



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
