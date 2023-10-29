use std::fs;
use toml::from_str;
use serde_derive::Deserialize;

const CONFIG_FILE: &str = "config.toml";


#[derive(Debug, Deserialize)]
pub struct Config {
    pub path_to_projects: String,
}

impl Config {

    pub fn default_path(self, app_name: &str) -> String {
        format!("{}{}/docker", self.path_to_projects, app_name)
    }
}


pub fn get_config() -> Config {
    let config_contents = match fs::read_to_string(CONFIG_FILE) {
        Ok(contents) => contents,
        Err(_) => {
            println!("Failed to read the config file: {}", CONFIG_FILE);
            "".to_owned()
        }
    };

    let config: Config = from_str(&config_contents).unwrap();

    config
}