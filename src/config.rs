use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tos_path: String,
    pub privacy_path: String,
    pub discord: DiscordConfig,
    pub webserver: WebConfig,
    pub interaction: IntConfig,
}

#[derive(Debug, Deserialize)]
pub struct DiscordConfig {
    pub guild_id: String,
    pub token: String,
    pub client_id: String,
    pub perm_int: i64,
    pub scope: String,
}

#[derive(Debug, Deserialize)]
pub struct WebConfig {
    pub listen_address: String,
    pub listen_port: u16,
}

#[derive(Debug, Deserialize)]
pub struct IntConfig {
    pub operators: Vec<i64>,
}

impl Config {
    pub fn read_from_file(path: &str) -> Self {
        let toml_str = match std::fs::read_to_string(path) {
            Ok(o) => o,
            Err(e) => panic!("Could not find config file: {}", e),
        };
        return toml::from_str(toml_str.as_str()).expect("Failed to parse config.");
    }
}
