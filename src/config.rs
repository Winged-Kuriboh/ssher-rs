use crate::model::Config;
use std::{fs, path::Path};

const CONFIG_PATH: &str = "~/.ssher.yaml";

pub(crate) fn load_config() -> Config {
    let path = shellexpand::tilde(CONFIG_PATH).into_owned();
    if Path::new(&path).exists() {
        let content = fs::read_to_string(path).expect("Failed to read config");
        serde_yaml::from_str(&content).expect("Failed to parse config")
    } else {
        Config { servers: vec![] }
    }
}

pub(crate) fn save_config(config: &Config) {
    let path = shellexpand::tilde(CONFIG_PATH).into_owned();
    let content = serde_yaml::to_string(config).expect("Failed to serialize config");
    fs::write(path, content).expect("Failed to save config");
}
