use crate::discord::{Context, Error, ids};
use poise::serenity_prelude as serenity;
use crate::database;



/// Add a user to the server cluster
#[poise::command(
    slash_command,
    rename="clusteradd",
)]
pub async fn cmd(
    ctx: Context<'_>,
    #[description = "User to add to cluster"] user: serenity::Member,
) -> Result<(), Error> {

    // let con = ctx.data().db.read().await.clone();
    let con = database::client::start_db().await;
    let userid = user.user.id.to_string();


    // Check if user in db
    let mut val = database::requests::user_in_cluster(con.clone(), &userid).await?;

    if let Some(val) = val.next().await? {
        let val: u32 = val.get(0)?;

        if val > 0 {
            ctx.send(poise::CreateReply::default()
                .content(format!("{} is already in cluster", user))
                .ephemeral(true)
            ).await?;

            return Ok(());
        }
    }


    database::requests::add_user(con.clone(), &userid).await?;
    user.add_role(ctx, serenity::RoleId::from(ids::CLUSTER_ROLE)).await?;

    ctx.say(format!("Added {} to cluster !", user)).await?;

    Ok(())

}
