use std::fs;
use wg_2024::config::Config;

pub fn load_from_file(path: &str) -> Config {
    let config_str = fs::read_to_string(path)
        .expect("ERROR[Network Initializer]: is unable to read config file 'config.toml' (please provide one in the root folder)");

    toml::from_str(&config_str).expect("ERROR[Network Initializer]: Unable to parse config.toml")
}
