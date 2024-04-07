extern crate alloc;

mod commands;
mod reactions;
mod create_event;
mod event_management;

use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents, EventHandler};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use std::env;
use regex::Regex;

struct Bot;
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;


fn set_env_var(env_var: &str, secret_store: &SecretStore) -> Result<(), Error> {
    let value = secret_store
        .get(env_var)
        .context(format!("Value not found in secrets: {}", env_var))?;
    env::set_var(env_var, value);
    Ok(())
}

fn set_env_variables(secret_store: &SecretStore) -> Result<(), Error> {
    let to_set = vec![
        "message_id_welcome",
        "role_id_flyght_member",
        "role_id_fmp",
        "emoji_fmp",
        "emoji_flyght_member"
    ];
    to_set.iter().map(|x| set_env_var(x, &secret_store)).collect()
}

async fn configure_discord(secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets_dev.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::generate_command_list(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::DIRECT_MESSAGE_REACTIONS
        | GatewayIntents::MESSAGE_CONTENT;

    let client = ClientBuilder::new(discord_token, intents)
        .event_handler(Bot)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}


#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    let _ = set_env_variables(&secret_store);
    configure_discord(secret_store).await
}
