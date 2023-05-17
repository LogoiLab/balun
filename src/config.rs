use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub tos_path: String,
    pub privacy_path: String,
    pub discord: DiscordConfig,
    pub webserver: WebConfig,
    pub interaction: IntConfig,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordConfig {
    pub guild_ids: Vec<i64>,
    pub token: String,
    pub client_id: String,
    pub perm_int: i64,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebConfig {
    pub listen_address: String,
    pub listen_port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntConfig {
    pub operators: Vec<i64>,
}

pub struct ConfigData;

impl serenity::prelude::TypeMapKey for ConfigData {
    type Value = Config;
}

impl Config {
    pub fn save(&self) {
        match std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
        {
            Ok(mut o) => {
                let t = toml::to_string(&self).unwrap();
                let toml = t.as_bytes();
                match o.write_all(toml) {
                    Ok(_) => (),
                    Err(e) => println!("Failed to write updates to config file: {}", e),
                }
                match o.flush() {
                    Ok(_) => (),
                    Err(e) => println!("Failed to flush changes to config file: {}", e),
                }
            }
            Err(e) => println!("Failed to open config file for updating: {}", e),
        }
    }
    pub fn read_from_file(path: &str) -> Self {
        let toml_str = match std::fs::read_to_string(path) {
            Ok(o) => o,
            Err(e) => panic!("Could not find config file: {}", e),
        };
        let mut config: Self = toml::from_str(toml_str.as_str()).expect("Failed to parse config.");
        config.path = path.into();
        return config;
    }
}
