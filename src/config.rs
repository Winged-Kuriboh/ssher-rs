use crate::model::Config;
use std::{fs, path::Path};

const CONFIG_PATH: &str = "~/.ssher.yaml";

pub(crate) fn load_config() -> anyhow::Result<Config> {
    let path = shellexpand::tilde(CONFIG_PATH).into_owned();
    if Path::new(&path).exists() {
        let content = fs::read_to_string(&path)
            .map_err(|e| anyhow::anyhow!("Failed to read config: {}", e))?;
        serde_yaml::from_str(&content).map_err(|e| anyhow::anyhow!("Failed to parse config: {}", e))
    } else {
        Ok(Config { servers: vec![] })
    }
}

pub(crate) fn save_config(config: &Config) -> anyhow::Result<()> {
    let path = shellexpand::tilde(CONFIG_PATH).into_owned();
    let content = serde_yaml::to_string(config)
        .map_err(|e| anyhow::anyhow!("Failed to serialize config: {}", e))?;
    fs::write(path, content).map_err(|e| anyhow::anyhow!("Failed to save config: {}", e))
}


