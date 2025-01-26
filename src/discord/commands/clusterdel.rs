use crate::discord::{Context, Error, ids};
use poise::serenity_prelude as serenity;
use crate::database;



/// Remove a user from the server cluster
#[poise::command(
    slash_command,
    rename="clusterdel",
)]
pub async fn cmd(
    ctx: Context<'_>,
    #[description = "User to remove from cluster"] user: serenity::Member,
) -> Result<(), Error> {

    // let con = ctx.data().db.read().await.clone();
    let con = database::client::start_db().await;
    let userid = user.user.id.to_string();


    // Check if user in db
    let mut val = database::requests::user_in_cluster(con.clone(), &userid).await?;

    if let Some(val) = val.next().await? {
        let val: u32 = val.get(0)?;

        if val > 0 {
            database::requests::remove_user(con.clone(), &userid).await?;
            user.remove_role(ctx, serenity::RoleId::from(ids::CLUSTER_ROLE)).await?;

            ctx.say(format!("Removed {} from cluster !", user)).await?;
            return Ok(());
        }
    }


    ctx.send(poise::CreateReply::default()
        .content(format!("{} is not in cluster", user))
        .ephemeral(true)
    ).await?;

    Ok(())

}
