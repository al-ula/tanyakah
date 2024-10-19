use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::sync::OnceLock;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use eyre::{eyre, Result};
use log::info;
use tracing::{error, warn};
use crate::db::TryGet;

pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub domain: String,
    pub db: PathBuf,
    pub assets: PathBuf,
    pub components: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8000,
            host: "0.0.0.0".to_string(),
            domain: "localhost".to_string(),
            db: PathBuf::from("db/data.db"),
            assets: PathBuf::from("assets"),
            components: PathBuf::from("components"),
        }
    }

}

impl TryGet<Config> for OnceLock<Config> {
    fn try_get(&self) -> Result<&Config> {
        match CONFIG.get() {
            None => {
                Err(eyre::eyre!("Config is not initialized"))
            }
            Some(c) => Ok(c),
        }
    }
}

impl Config {
    pub fn init() -> Result<()> {
        match config_fromenv() {
            Ok(config) => {
                CONFIG.set(config).ok();
                info!("Config loaded from env variable");
                Ok(())
            }
            Err(e) => {
                warn!("Failed to load config env variable: {}", e);
                match config_appdata() {
                    Ok(config) => {
                        CONFIG.set(config).ok();
                        info!("Config loaded from default config directory");
                        Ok(())
                    }
                    Err(e) => {
                        error!("Failed to load config folder: {}", e);
                        let config = Config::default();
                        warn!("Using default config");
                        CONFIG.set(config).ok();
                        Ok(())
                    }
                }
            }
        }
    }
}

fn config_fromenv() -> Result<Config> {
    let from_env = std::env::var("TANYAKAH_CONF")?;
    let mut file = fs::File::open(from_env)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}

fn config_appdata() -> Result<Config> {
    let config_file_path: PathBuf;
    if let Some(proj_dirs) = ProjectDirs::from("me", "iisa", "tanyakah") {
        let config_dir = proj_dirs.config_dir();
        config_file_path = config_dir.join("config.toml");
    } else {
        return Err(eyre!("Config directory could not be determined."));
    }
    // Check if the config file exists, and load it if it does
    if config_file_path.exists() {
        let config_data = fs::read_to_string(&config_file_path)?;
        let config: Config = toml::from_str(&config_data)?;
        Ok(config)
    } else {
        // If the file doesn't exist, use the default config
        Err(eyre!("Config file not found, using default values."))
    }
}