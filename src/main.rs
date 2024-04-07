extern crate alloc;
extern crate core;

mod reactions;
mod config_data;

use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

struct Bot;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;


async fn configure_discord(secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets_dev.toml`
    let discord_configs = config_data::ConfigData::new(&secret_store);
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework: poise::Framework<Data, Error> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
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
        .type_map_insert::<config_data::ConfigData>(discord_configs)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}


#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    configure_discord(secret_store).await
}
