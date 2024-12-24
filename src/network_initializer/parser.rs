use std::fs;
use wg_2024::config::Config;

pub fn load_from_file(path: &str) -> Config {
    let config_str = fs::read_to_string(path).expect("Unable to read config file");
    toml::from_str(&config_str).expect("Unable to parse TOML")
}
