use crate::consts;
use crate::discord::{commands, framework, ids, Data, Handler};
use poise::serenity_prelude as serenity;

pub async fn app() {
    let token = std::env::var(format!("DISCORD_TOKEN_{}", consts::mode().to_uppercase()))
        .expect("missing or wrong DISCORD_TOKEN_???");

    let intents = serenity::GatewayIntents::GUILD_MEMBERS | serenity::GatewayIntents::GUILDS;

    let status = serenity::OnlineStatus::Online;
    let activity = serenity::ActivityData::playing(consts::version());

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::ping::cmd(),
            commands::embed::cmd(),
            commands::clusteradd::cmd(),
            commands::clusterdel::cmd(),
            commands::clusterlist::cmd(),
            commands::clusterupdate::cmd(),
        ],

        post_command: |ctx| Box::pin(framework::post_command(ctx)),
        on_error: |err| Box::pin(framework::on_error(err)),
        command_check: Some(|ctx| Box::pin(framework::command_check(ctx))),

        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let main_guild = serenity::GuildId::from(ids::GUILD_ID);

                // poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                // for cmd in main_guild.get_commands(ctx).await.unwrap() {
                //     main_guild.delete_command(ctx, cmd.id).await.unwrap();
                // }

                poise::builtins::register_in_guild(ctx, &framework.options().commands, main_guild)
                    .await?;

                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .status(status)
        .activity(activity)
        .event_handler(Handler)
        .await;

    client.unwrap().start().await.unwrap();
}
