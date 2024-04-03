use serenity::all::GuildId;

use serde::{Deserialize, Serialize};

use std::fs::{File, DirBuilder, OpenOptions};
use std::path::Path;

use std::io::{Read, Write};

static CONFIG_PATH: &str = "/home/markg/.config/dart-sweeper";

#[derive(Deserialize, Serialize)]
pub struct BotConfig {
    pub whitelisted_roles: Vec<String>,
    pub mod_log_channel_id: Option<u64>,
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            whitelisted_roles: vec!["Verified".into()],
            mod_log_channel_id: Default::default(),
        }
    }
}

pub fn get_config(guild: GuildId) -> BotConfig {
    let path = Path::new(CONFIG_PATH);
    let path = path.join(guild.get().to_string());

    println!("Path is {:?}", path);

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_e) => {
            // Create the directory
            DirBuilder::new().recursive(true).create(CONFIG_PATH).unwrap();
            // Create the file
            let mut file = File::create(&path).unwrap();

            // Write the default configuration to the file
            let file_contents = toml::to_string(&BotConfig::default()).unwrap();
            file.write(file_contents.as_bytes()).unwrap();
            
            return BotConfig::default();
        }
    };

    // Read and parse the config file
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    toml::from_str(&s).unwrap()
}
