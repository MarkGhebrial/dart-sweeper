use serenity::all::GuildId;

pub struct BotConfig {
    pub whitelisted_roles: Vec<String>,
    pub mod_log_channel_id: Option<u64>,
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            whitelisted_roles: Default::default(),
            mod_log_channel_id: Default::default(),
        }
    }
}

pub fn get_config(guild: GuildId) -> BotConfig {
    match guild.get() {
        1042992678469632031 => BotConfig {
            whitelisted_roles: vec!["Verified".into(), "Trusted".into()],
            mod_log_channel_id: Some(1224851810200584222),
        },
        _ => BotConfig {
            whitelisted_roles: vec!["Verified".into()],
            mod_log_channel_id: None,
        },
    }
}
