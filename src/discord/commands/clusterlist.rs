use crate::discord::{Context, Error, default};
use crate::database;



/// List all users in the server cluster
#[poise::command(
    slash_command,
    rename="clusterlist",
    default_member_permissions="ADMINISTRATOR",
)]
pub async fn cmd(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let text = format!("⌛ Loading...");
    let response = ctx.say(text).await?;

    let con = database::client::start_db().await;
    let mut users = database::requests::retrieve_users(con).await.unwrap();
    let mut user_ids: Vec<String> = Vec::new();

    while let Some(users) = users.next().await? {
        let uid: String = users.get(0)?;
        user_ids.push(uid)
    }

    let user_ids = user_ids.iter()
        .map(|x| format!("<@{}>", x))
        .collect::<Vec<String>>()
        .join("\n");

    let embed = default::embed()
        .title("Users in Cluster")
        .description(format!("{}", user_ids));

    let msg = poise::CreateReply::default()
        .content("")
        .embed(embed);

    response.edit(ctx, msg).await?;

    Ok(())
}
