use std::fs;
use wg_2024::config::Config;

/// Try to parse a file into a Config.
/// # Errors
/// - Unable to read the file
/// - Unable to parse to Config struct
pub fn load_from_file(path: &str) -> Result<Config, String> {
    let config_str = fs::read_to_string(path)
        .map_err(|_| "Unable to read config from '{path}' relative from the root of the project")?;

    Ok(toml::from_str(&config_str).map_err(|_| "Unable to parse config.toml")?)
}
