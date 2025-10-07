use std::{fs::File, io::Read};

use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Deserialize)]
pub struct DDNSConfig {
    pub domain: String,
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "domain")]
    pub domains: Vec<DDNSConfig>,
}

impl Config {
    pub fn read(path: &str) -> Result<Config> {
        let mut config_buffer: String = "".to_string();
        let mut config_file =
            File::open(path).expect(format!("config file {} does not exist", path).as_str());
        config_file.read_to_string(&mut config_buffer)?;
        let config: Config = toml::from_str(&config_buffer).expect("failed to parse config toml");
        Ok(config)
    }
}
