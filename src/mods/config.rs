use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    pub static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::new()));
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub subnet_prefix_length: u8,
    pub upper_interface: String,
    pub next_interface: String,
    pub features: Features,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Features {
    pub radvd: bool,
    pub dhcpv6: bool,
    pub ndproxy: bool,
    pub config_interface: bool,
    pub router: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            subnet_prefix_length: 64,
            upper_interface: String::from(""),
            next_interface: String::from(""),
            features: Features {
                radvd: true,
                dhcpv6: true,
                ndproxy: true,
                config_interface: true,
                router: true,
            },
        }
    }

    pub fn load<T: AsRef<Path>>(&mut self, path: &T) -> Result<Config, String> {
        let config_file = std::fs::read_to_string(path.as_ref()).map_err(|e| e.to_string())?;
        let config: Config = serde_json::from_str(&config_file).map_err(|e| e.to_string())?;
        *self = config.clone();
        Ok(config)
    }
}
