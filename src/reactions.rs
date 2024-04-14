use std::time::Duration;
use async_std::task::sleep;
use async_trait::async_trait;
use chrono;
use serenity::all::{ChannelId, Context, EditMessage, EventHandler, Message, MessageId, Reaction, Ready};
use tracing::{error, info};
use crate::Bot;
use crate::config_data::ConfigData;

#[async_trait]
impl EventHandler for Bot {
    /// Predominantly used as a test function. Will reply with a greeting to anyone that puts !Hello
    /// in the chat
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!Hello" {
            let content = format!("Hello {}", msg.author);
            if let Err(e) = msg.channel_id.say(&ctx.http, content).await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    /// Function to run when a new reaction is detected
    /// Should assign roles based on reactions to welcome message
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        let data = ctx.data.read().await;
        let configs = data.get::<ConfigData>().unwrap();
        if add_reaction.message_id.get() == configs.message_ids.welcome_message {
            if add_reaction.emoji.unicode_eq(&configs.emoji_ids.fmp) {
                add_reaction.member.unwrap().add_role(ctx.http, configs.role_ids.fmp).await.unwrap();
            } else if add_reaction.emoji.unicode_eq(&configs.emoji_ids.flyght_member) {
                add_reaction.member.unwrap().add_role(ctx.http, configs.role_ids.flyght_member).await.unwrap();
            }
        }
    }

    /// Function to run when a reaction is removed
    /// Should remove roles based on removed reaction to welcome message
    async fn reaction_remove(&self, ctx: Context, removed_reaction: Reaction) {
        async fn remove_role(ctx: &Context, removed_reaction: Reaction, role: u64) {
            let user_id = removed_reaction.user_id.expect("UserID should exist");
            let server = removed_reaction.guild_id.expect("Server ID should exist");
            let member = server.member(&ctx.http, user_id).await.unwrap();
            member.remove_role(&ctx.http, role).await.unwrap();
        }
        let data = ctx.data.read().await;
        let configs = data.get::<ConfigData>().unwrap();
        if removed_reaction.message_id.get() == configs.message_ids.welcome_message {
            if removed_reaction.emoji.unicode_eq(&configs.emoji_ids.fmp) {
                remove_role(&ctx, removed_reaction, configs.role_ids.fmp).await;
            } else if removed_reaction.emoji.unicode_eq(&configs.emoji_ids.flyght_member) {
                remove_role(&ctx, removed_reaction, configs.role_ids.flyght_member).await;
            }
        }
    }

    /// Function triggers when the application connects
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        let data = ctx.data.read().await;
        let configs = data.get::<ConfigData>().unwrap();
        let message_id = MessageId::from(configs.message_ids.bot_message);
        let channel_id = ChannelId::from(configs.channel_ids.bot_management);
        let message = ctx.http.get_message(channel_id, message_id).await.unwrap();
        let current_time = chrono::offset::Utc::now();
        let content = format!("Bot started up at: {}", current_time);
        let s_message = message.reply(&ctx.http, content).await.expect("Reply should work");
        let content = format!("Bot checking in at: {}", current_time);
        let mut c_message = s_message.reply(&ctx.http, content).await.expect("Reply should work");

        loop {
            sleep(Duration::from_secs(10)).await;
            let current_time = chrono::offset::Utc::now();
            let content = format!("Bot checking in at: {}", current_time);
            let mut msg_content = EditMessage::new();
            msg_content = msg_content.content(content);
            c_message.edit(&ctx.http, msg_content).await.expect("Reply should work")
        }
    }
}