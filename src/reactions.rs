use async_trait::async_trait;
use serenity::all::{Context, EventHandler, Message, Reaction, Ready, RoleId};
use tracing::{error, info};
use crate::Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!howdy" {
            println!("A user said howdy!");
            if let Err(e) = msg.channel_id.say(&ctx.http, "worldio!").await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    /// Function to run when a new reaction is detected
    /// Should assign roles based on reactions to welcome message
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        if add_reaction.message_id.get() == 1223341046214299749 {
            let fmp_role = 1222939701162278912;
            add_reaction.member.unwrap().add_role(ctx.http, fmp_role).await.unwrap();
            println!("FMP role added to user")
        }
    }

    /// Function to run when a reaction is removed
    /// Should remove roles based on removed reaction to welcome message
    async fn reaction_remove(&self, ctx: Context, removed_reaction: Reaction) {
        if removed_reaction.message_id.get() == 1223341046214299749 {
            let fmp_role = 1222939701162278912;
            let user_id = removed_reaction.user_id.expect("UserID should exist");
            let server = removed_reaction.guild_id.expect("Server ID should exist");
            let member = server.member(&ctx.http, user_id).await.unwrap();
            member.remove_role(&ctx.http, fmp_role).await.unwrap();
            println!("FMP role removed from user");
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}