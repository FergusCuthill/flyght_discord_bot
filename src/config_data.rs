use serenity::prelude::TypeMapKey;
use shuttle_runtime::SecretStore;


/// Channel used for managing the bot
#[derive(Debug)]
pub struct ChannelIDs {
    pub bot_management: u64
}


/// Struct lists each role ID the app has
#[derive(Debug)]
pub struct RoleIDs {
    pub admin: u64,
    pub committee: u64,
    pub indoor_lead: u64,
    pub outdoor_lead: u64,
    pub fmp: u64,
    pub flyght_member: u64
}


/// Struct for set message IDs such as welcome message which is where the bot checks for role
/// assignment reactions
#[derive(Debug)]
pub struct MessageIDs {
    pub welcome_message: u64,
    pub bot_message: u64
}


/// Struct for emoji IDs.
#[derive(Debug)]
pub struct EmojiIDs {
    pub fmp: String,
    pub flyght_member: String
}

/// Struct containing config data to be included in the Context argument for receiving discord
/// messages.
#[derive(Debug)]
pub struct ConfigData {
    pub channel_ids: ChannelIDs,
    pub message_ids: MessageIDs,
    pub role_ids: RoleIDs,
    pub emoji_ids: EmojiIDs
}

impl ConfigData {
    /// Creates a new instance of ConfigData from values contained in the secrets store
    pub fn new(secret_store: &SecretStore) -> ConfigData {
        let channel_ids = ChannelIDs {
            bot_management: secret_store.get("channel_id_bot").unwrap().parse().unwrap()
        };
        let role_ids = RoleIDs {
            admin: secret_store.get("role_id_admin").unwrap().parse().unwrap(),
            committee: secret_store.get("role_id_committee").unwrap().parse().unwrap(),
            indoor_lead: secret_store.get("role_id_indoor_lead").unwrap().parse().unwrap(),
            outdoor_lead: secret_store.get("role_id_outdoor_lead").unwrap().parse().unwrap(),
            fmp: secret_store.get("role_id_fmp").unwrap().parse().unwrap(),
            flyght_member: secret_store.get("role_id_flyght_member").unwrap().parse().unwrap()
        };
        let message_ids = MessageIDs {
            welcome_message: secret_store.get("message_id_welcome").unwrap().parse().unwrap(),
            bot_message: secret_store.get("message_id_bot").unwrap().parse().unwrap()
        };
        let emoji_ids = EmojiIDs {
            fmp: "🙋‍♀️".to_string(),
            flyght_member: "✅".to_string()
        };
        ConfigData {channel_ids, role_ids, message_ids, emoji_ids}
    }
}

/// Allows the Config data type to be included in a TypeMap - this is how it is passed as part of
/// the Context.data argument for receiving discord events.
impl TypeMapKey for ConfigData {
    type Value = ConfigData;
}
