use std::fs;
use std::path::Path;
use wg_2024::config::Config;

pub fn load_from_file(path: &str) -> Config {
    let config_str = fs::read_to_string(path).expect(&format!(
        "ERROR[Network Initializer]: is unable to read config from '{}' relative from path = {:?}",
        path,
        Path::new(".").canonicalize()
    ));

    toml::from_str(&config_str).expect("ERROR[Network Initializer]: Unable to parse config.toml")
}
