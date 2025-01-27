use serenity::async_trait;
use poise::serenity_prelude as serenity;
use crate::discord::{Handler, ids, logging};


#[async_trait]
impl serenity::EventHandler for Handler {
    async fn guild_member_addition(
        &self,
        ctx: serenity::Context,
        new_member: serenity::Member,
    ) {
        let role = serenity::RoleId::from(ids::MEMBER_ROLE);
        let _ = new_member.add_role(&ctx, role).await;

        let msg = format!("Given role <@&{}> to {}", role, new_member);

        logging::log_to_discord(&ctx, msg, logging::LogRole::Info).await;
        
    }
}

