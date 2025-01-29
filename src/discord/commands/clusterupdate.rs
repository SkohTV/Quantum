use poise::serenity_prelude as serenity;
use crate::discord::{Context, Error, ids};
use crate::database;
use std::str::FromStr;



/// Remove and add back all cluster roles, for a refresh in case of bugs
#[poise::command(
    slash_command,
    rename="clusterupdate",
    default_member_permissions="ADMINISTRATOR",
)]
pub async fn cmd(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let text = format!("⌛ Loading...");
    let response = ctx.say(text).await?;

    let cluster_role = serenity::RoleId::from(ids::CLUSTER_ROLE);
    let guild = ctx.guild().unwrap().clone();

    let con = database::client::start_db().await;
    let mut cluster_members = database::requests::retrieve_users(con).await.unwrap();

    while let Some(user) = cluster_members.next().await? {
        let uid: String = user.get(0)?;
        let uid = serenity::UserId::from_str(uid.as_str()).unwrap();

        if let Ok(member) = guild.member(ctx, uid).await {
            member.add_role(ctx, cluster_role).await?;
        }
    }

    let server_members = guild.members(ctx, None, None).await?;

    for user in &server_members {
        let roles = user.roles(ctx)
            .unwrap()
            .iter()
            .map(|x| x.id)
            .collect::<Vec<_>>();

        if roles.contains(&cluster_role) {
            user.remove_role(ctx, cluster_role).await?;
        }
    }

    let msg = poise::CreateReply::default()
        .content(format!("👍 All roles for users in cluster have been refreshed"));

    response.edit(ctx, msg).await?;


    Ok(())
}
