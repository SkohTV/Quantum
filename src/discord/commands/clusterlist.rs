use crate::discord::{Context, Error, default};
use crate::database;



/// List all users in the server cluster
#[poise::command(
    slash_command,
    rename="clusterlist",
)]
pub async fn cmd(
    ctx: Context<'_>,
) -> Result<(), Error> {

    // let con = ctx.data().db.read().await.clone();
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
        .description(format!("{}", user_ids));

    let msg = poise::CreateReply::default()
        .embed(embed);

    ctx.send(msg).await?;

    Ok(())
}
