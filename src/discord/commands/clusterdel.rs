use crate::discord::{Context, Error, ids};
use poise::serenity_prelude as serenity;
use crate::database;



/// Remove a user from the server cluster
#[poise::command(
    slash_command,
    rename="clusterdel",
    default_member_permissions="ADMINISTRATOR",
)]
pub async fn cmd(
    ctx: Context<'_>,
    #[description = "User to remove from cluster"] user: serenity::Member,
) -> Result<(), Error> {

    let text = format!("⌛ Loading...");
    let response = ctx.say(text).await?;

    let con = database::client::start_db().await;
    let userid = user.user.id.to_string();


    // Check if user in db
    let mut val = database::requests::user_in_cluster(con.clone(), &userid).await?;

    if let Some(val) = val.next().await? {
        let val: u32 = val.get(0)?;

        if val > 0 {
            database::requests::remove_user(con.clone(), &userid).await?;
            user.remove_role(ctx, serenity::RoleId::from(ids::CLUSTER_ROLE)).await?;

            let msg = poise::CreateReply::default()
                .content(format!("Removed {} from cluster !", user));

            response.edit(ctx, msg).await?;

            return Ok(());
        }
    }


    let msg = poise::CreateReply::default()
        .content(format!("{} is not in cluster", user))
        .ephemeral(true);

    response.edit(ctx, msg).await?;

    Ok(())

}
