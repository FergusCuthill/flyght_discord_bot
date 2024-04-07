use serenity::prelude::TypeMapKey;
use shuttle_runtime::SecretStore;

#[derive(Debug)]
pub struct RoleIDs {
    pub admin: u64,
    pub committee: u64,
    pub indoor_lead: u64,
    pub outdoor_lead: u64,
    pub fmp: u64,
    pub flyght_member: u64
}

#[derive(Debug)]
pub struct MessageIDs {
    pub welcome_message: u64
}

#[derive(Debug)]
pub struct EmojiIDs {
    pub fmp: String,
    pub flyght_member: String
}

#[derive(Debug)]
pub struct ConfigData {
    pub message_ids: MessageIDs,
    pub role_ids: RoleIDs,
    pub emoji_ids: EmojiIDs
}

impl ConfigData {
    pub fn new(secret_store: &SecretStore) -> ConfigData {
        let role_ids = RoleIDs {
            admin: secret_store.get("role_id_admin").unwrap().parse().unwrap(),
            committee: secret_store.get("role_id_committee").unwrap().parse().unwrap(),
            indoor_lead: secret_store.get("role_id_indoor_lead").unwrap().parse().unwrap(),
            outdoor_lead: secret_store.get("role_id_outdoor_lead").unwrap().parse().unwrap(),
            fmp: secret_store.get("role_id_fmp").unwrap().parse().unwrap(),
            flyght_member: secret_store.get("role_id_flyght_member").unwrap().parse().unwrap()
        };
        let message_ids = MessageIDs {
            welcome_message: secret_store.get("message_id_welcome").unwrap().parse().unwrap()
        };
        let emoji_ids = EmojiIDs {
            fmp: "🙋‍♀️".to_string(),
            flyght_member: "✅".to_string()
        };
        ConfigData {role_ids, message_ids, emoji_ids}
    }
}

impl TypeMapKey for ConfigData {
    type Value = ConfigData;
}
